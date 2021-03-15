initSidebarItems({"struct":[["ChanAbort","Abort an in-progress transfer sequence on one or more channels"],["CtrlTrig","DMA Channel 6 Control and Status"],["DbgCtdreq","Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."],["FifoLevels","Debug RAF, WAF, TDF levels"],["Inte0","Interrupt Enables for IRQ 0"],["Inte1","Interrupt Enables for IRQ 1"],["Intf0","Force Interrupts"],["Intf1","Force Interrupts for IRQ 1"],["Intr","Interrupt Status (raw)"],["Ints0","Interrupt Status for IRQ 0"],["Ints1","Interrupt Status (masked) for IRQ 1"],["MultiChanTrigger","Trigger one or more channels simultaneously"],["NChannels","The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."],["SniffCtrl","Sniffer Control"],["Timer","Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]]});