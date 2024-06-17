#define PREFIX64 aarch64-none-elf-
#define AARCH 64
#define RASPPI 4

#include "circle/actled.h"
#include "circle/alloc.h"
#include "circle/devicenameservice.h"
#include "circle/exceptionhandler.h"
#include "circle/fs/fat/fatfs.h"
#include "circle/gpiopin.h"
#include "circle/interrupt.h"
#include "circle/koptions.h"
#include "circle/logger.h"
#include "circle/pwmoutput.h"
#include "circle/screen.h"
#include "circle/serial.h"
#include "circle/spimaster.h"
#include "circle/startup.h"
#include "circle/timer.h"
#include "circle/usb/usbhcidevice.h"