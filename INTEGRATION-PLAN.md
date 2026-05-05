# rsh Integration Plan: Making rsh the Default Serix Shell

## Problem Statement

rsh (Rust Shell) is a feature-complete userspace shell built for bare-metal Serix kernel execution. It compiles successfully and has all infrastructure in place, but is not yet embedded in the kernel or accessible at boot. The goal is to make rsh the default interactive shell by embedding it in the kernel and having the boot process execute it.

## Proposed Approach

**Phase 1: Kernel Integration (Enable rsh accessibility)**
- Embed rsh binary in kernel via `include_bytes!()` (like ext4d)
- Register rsh in VFS as a RamFile at boot
- Update Makefile to build rsh before kernel

**Phase 2: Feature Validation (Ensure rsh works end-to-end)**
- Boot kernel and verify rsh REPL appears
- Test all rsh builtins work in kernel context
- Identify and fix any syscall/functionality gaps

**Phase 3: Polish & Documentation**
- Update documentation (ROADMAP, README)
- Verify graceful error handling
- Clean up any init/boot output

## System Architecture

### Current Boot Flow
```
Bootloader → Kernel init → ext4d daemon → init process → [init spawns rsh]
```

### Key Components
- **rsh (56KB static ELF)**: Userspace REPL with builtins (help, echo, set, get, if, repeat, history, exit, clear)
- **ulib**: Provides syscall wrappers + I/O (ulib::println!, ulib::io::read_line())
- **Kernel VFS**: Supports embedding binaries as RamFile nodes
- **Process spawning**: Kernel can spawn /rsh as a user process

### Syscall Requirements
rsh uses:
- `ulib::io::read_line()` → **READ(0)** syscall
- `ulib::println!()` → **WRITE(1)** syscall
- Shell state: in-process (no file I/O needed for builtins)
- All syscalls already supported by kernel ✓

## Implementation Todos

### Phase 1: Kernel Integration
1. **kernel-embed-rsh-binary**: Add RSH_ELF constant in kernel/src/main.rs after line 54
   - Pattern: `static RSH_ELF: &[u8] = include_bytes!("../../target/x86_64-unknown-none/release/rsh");`
   - Must match path where `make rsh` outputs the binary

2. **kernel-register-rsh-vfs**: Insert rsh into "/" filesystem during boot (~line 558 in main.rs)
   - Pattern: Copy ext4d insertion block (lines 549-552)
   - Create RamFile with RSH_ELF data
   - Insert into root directory with key "rsh"
   - No IPC ports needed (unlike ext4d)

3. **makefile-add-rsh-dependency**: Update Makefile line 17
   - Current: `kernel: ext4d`
   - Change to: `kernel: ext4d rsh`
   - Ensures rsh builds before kernel compilation

4. **clean-test-build**: Run full build cycle
   - `make clean && cargo clean`
   - `make iso`
   - Verify no build errors

### Phase 2: Feature Validation
5. **boot-and-test**: Run kernel with QEMU
   - `make run` (boots in QEMU with serial stdio)
   - Verify kernel starts, ext4d spawns, init spawns
   - Check if rsh REPL prompt appears

6. **test-rsh-builtins**: Manually verify all commands work
   - `help` → prints command list
   - `echo X Y Z` → prints "X Y Z"
   - `set VAR value` + `get VAR` → variable storage
   - `history` → command history works
   - `exit` → cleanly terminates rsh

7. **test-shell-io**: Verify I/O subsystem works
   - Keyboard input appears in REPL
   - Output renders on framebuffer/serial

8. **identify-gaps**: Document any missing functionality
   - Note any syscalls rsh tries to use but kernel doesn't support
   - Identify any hardware-specific issues (keyboard, framebuffer)

### Phase 3: Polish
9. **update-documentation**: Update docs/ROADMAP.md and README.md
   - Mark rsh as default shell
   - Document how to build/run
   - Add to feature list if not already there

10. **verify-graceful-shutdown**: Ensure rsh exits cleanly
    - Test `exit 0` and `exit 1`
    - Verify init handles rsh termination correctly

11. **code-review**: Final check
    - Ensure code style matches (tabs, comments, naming)
    - Verify no warnings introduced
    - Confirm Makefile targets are correct

## Technical Considerations

### Memory Impact
- rsh binary: 56KB
- Embedded in kernel → added to ISO size
- Negligible impact (~56KB added to ISO)

### VFS Registration Timing
- rsh must be registered after VFS root is created (after graphics init)
- Must happen before init is spawned (init will try to find /rsh)
- Current sequence (lines 540-560 in main.rs) is correct for this

### Init vs Direct Spawn
- **Decision**: Keep init as boot manager, have it spawn rsh
  - Reason: Maintains clean boot flow, allows init to set up before rsh
  - Alternative: Could replace init with rsh directly (out of scope for now)

### Potential Issues & Mitigations
| Issue | Mitigation |
|-------|-----------|
| Binary embedding path wrong | Verify path matches Makefile output location |
| RamFile doesn't persist across context switches | Use Arc/Mutex pattern like ext4d (already provided) |
| rsh binary path not found by init | Verify "rsh" key in VFS, check /rsh lookup logic |
| Keyboard input not reaching rsh | Test with init echo loop first, then rsh |
| Output not rendering | Check framebuffer setup before spawning rsh |

## Success Criteria

- [ ] `make iso` completes without errors
- [ ] Kernel boots to rsh REPL prompt
- [ ] All rsh builtins execute without panicking
- [ ] Keyboard input works and is echoed
- [ ] `exit` cleanly terminates rsh process
- [ ] Serial and framebuffer output synchronized
- [ ] No new compiler warnings introduced

## Files to Modify

| File | Change | Lines |
|------|--------|-------|
| kernel/src/main.rs | Add RSH_ELF constant | ~54 |
| kernel/src/main.rs | Insert rsh into VFS | ~558 |
| Makefile | Add rsh dependency | 17 |
| docs/ROADMAP.md | Mark rsh as done | (if tracking) |

## Out of Scope (Future Work)

- Piping between commands (rsh → other processes)
- File descriptor redirection in rsh
- Command chaining or backgrounding
- Filesystem I/O from rsh (read/write files)
- Performance optimization
- Terminal emulation (color codes, etc.)

## Rollback Plan

If issues occur:
1. Remove rsh from kernel (comment out RSH_ELF and insertion code)
2. Revert Makefile change
3. Fall back to kshell (kernel-space shell, already functional)
4. Investigate issue and iterate

---

## Status: Ready for Implementation

All prerequisites met:
- ✓ rsh binary builds cleanly
- ✓ Makefile build target exists
- ✓ Kernel infrastructure supports embedding (proven by ext4d)
- ✓ init already attempts to spawn /rsh
- ✓ All required syscalls available in kernel
