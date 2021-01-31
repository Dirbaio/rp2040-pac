initSidebarItems({"struct":[["DivCsr","Control and status register for divider."],["FifoSt","Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."],["GpioHiIn","Input value for QSPI pins"],["GpioHiOe","QSPI output enable"],["GpioHiOeClr","QSPI output enable clear"],["GpioHiOeSet","QSPI output enable set"],["GpioHiOeXor","QSPI output enable XOR"],["GpioHiOut","QSPI output value"],["GpioHiOutClr","QSPI output value clear"],["GpioHiOutSet","QSPI output value set"],["GpioHiOutXor","QSPI output value XOR"],["GpioIn","Input value for GPIO pins"],["GpioOe","GPIO output enable"],["GpioOeClr","GPIO output enable clear"],["GpioOeSet","GPIO output enable set"],["GpioOeXor","GPIO output enable XOR"],["GpioOut","GPIO output value"],["GpioOutClr","GPIO output value clear"],["GpioOutSet","GPIO output value set"],["GpioOutXor","GPIO output value XOR"],["Interp0Accum0Add","Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."],["Interp0Accum1Add","Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."],["Interp0CtrlLane0","Control register for lane 0"],["Interp0CtrlLane1","Control register for lane 1"],["Interp1Accum0Add","Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."],["Interp1Accum1Add","Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."],["Interp1CtrlLane0","Control register for lane 0"],["Interp1CtrlLane1","Control register for lane 1"]]});