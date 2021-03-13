initSidebarItems({"struct":[["Ctrl","PIO control register"],["DbgCfginfo","The PIO hardware has some free parameters that may vary between chip products. These should be provided in the chip datasheet, but are also exposed here."],["Fdebug","FIFO debug register"],["Flevel","FIFO levels"],["Fstat","FIFO status register"],["InstrMem","Write-only access to instruction memory location 27"],["Intr","Interrupt Enable for irq0"],["Irq","Interrupt request register. Write 1 to clear"],["IrqForce","Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."],["SmAddr","Current instruction address of state machine 0"],["SmClkdiv","Clock divider register for state machine 3 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"],["SmExecctrl","Execution/behavioural settings for state machine 3"],["SmInstr","Instruction currently being executed by state machine 0 Write to execute an instruction immediately (including jumps) and then resume execution."],["SmPinctrl","State machine pin control"],["SmShiftctrl","Control behaviour of the input/output shift registers for state machine 1"]]});