initSidebarItems({"struct":[["AddrEndp","Device address and endpoint control"],["AddrEndp1","Interrupt endpoint 1. Only valid for HOST mode."],["AddrEndp10","Interrupt endpoint 10. Only valid for HOST mode."],["AddrEndp11","Interrupt endpoint 11. Only valid for HOST mode."],["AddrEndp12","Interrupt endpoint 12. Only valid for HOST mode."],["AddrEndp13","Interrupt endpoint 13. Only valid for HOST mode."],["AddrEndp14","Interrupt endpoint 14. Only valid for HOST mode."],["AddrEndp15","Interrupt endpoint 15. Only valid for HOST mode."],["AddrEndp2","Interrupt endpoint 2. Only valid for HOST mode."],["AddrEndp3","Interrupt endpoint 3. Only valid for HOST mode."],["AddrEndp4","Interrupt endpoint 4. Only valid for HOST mode."],["AddrEndp5","Interrupt endpoint 5. Only valid for HOST mode."],["AddrEndp6","Interrupt endpoint 6. Only valid for HOST mode."],["AddrEndp7","Interrupt endpoint 7. Only valid for HOST mode."],["AddrEndp8","Interrupt endpoint 8. Only valid for HOST mode."],["AddrEndp9","Interrupt endpoint 9. Only valid for HOST mode."],["BuffCpuShouldHandle","Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."],["BuffStatus","Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."],["EpAbort","Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."],["EpAbortDone","Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."],["EpStallArm","Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."],["EpStatusStallNak","Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."],["IntEpCtrl","interrupt endpoint control register"],["Inte","Interrupt Enable"],["Intf","Interrupt Force"],["Intr","Raw Interrupts"],["Ints","Interrupt status after masking & forcing"],["MainCtrl","Main control register"],["NakPoll","Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."],["SieCtrl","SIE control register"],["SieStatus","SIE status register"],["SofRd","Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."],["SofWr","Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."],["UsbMuxing","Where to connect the USB controller. Should be to_phy by default."],["UsbPwr","Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value."],["UsbphyDirect","This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit."],["UsbphyDirectOverride","Override enable for each control in usbphy_direct"],["UsbphyTrim","Used to adjust trim values of USB phy pull down resistors."]]});