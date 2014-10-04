// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::{ObjCMethodCall, id, SEL, NSInteger, NSUInteger};

pub type CGFloat = f32;

#[repr(C)]
pub struct NSPoint {
    pub x: f64,
    pub y: f64,
}

impl NSPoint {
    #[inline]
    pub fn new(x: f64, y: f64) -> NSPoint {
        NSPoint {
            x: x,
            y: y,
        }
    }
}

#[repr(C)]
pub struct NSSize {
    pub width: f64,
    pub height: f64,
}

impl NSSize {
    #[inline]
    pub fn new(width: f64, height: f64) -> NSSize {
        NSSize {
            width: width,
            height: height,
        }
    }
}

#[repr(C)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
}

impl NSRect {
    #[inline]
    pub fn new(origin: NSPoint, size: NSSize) -> NSRect {
        NSRect {
            origin: origin,
            size: size
        }
    }
}

#[link(name = "AppKit", kind = "framework")]
extern {}

#[link(name = "Foundation", kind = "framework")]
extern {
    pub static NSDefaultRunLoopMode: id;
}

pub unsafe fn NSApp() -> id {
    "NSApplication".send("sharedApplication", ())
}

#[repr(i64)]
pub enum NSApplicationActivationPolicy {
    NSApplicationActivationPolicyRegular = 0,
    NSApplicationActivationPolicyERROR = -1
}

#[repr(u64)]
pub enum NSWindowMask {
    NSBorderlessWindowMask      = 0,
    NSTitledWindowMask          = 1 << 0,
    NSClosableWindowMask        = 1 << 1,
    NSMiniaturizableWindowMask  = 1 << 2,
    NSResizableWindowMask       = 1 << 3,

    NSTexturedBackgroundWindowMask  = 1 << 8,

    NSUnifiedTitleAndToolbarWindowMask  = 1 << 12,

    NSFullScreenWindowMask      = 1 << 14
}

#[repr(u64)]
pub enum NSBackingStoreType {
    NSBackingStoreRetained      = 0,
    NSBackingStoreNonretained   = 1,
    NSBackingStoreBuffered      = 2
}

#[repr(u64)]
pub enum NSOpenGLPixelFormatAttribute {
    NSOpenGLPFAAllRenderers             = 1,
    NSOpenGLPFATripleBuffer             = 3,
    NSOpenGLPFADoubleBuffer             = 5,
    NSOpenGLPFAStereo                   = 6,
    NSOpenGLPFAAuxBuffers               = 7,
    NSOpenGLPFAColorSize                = 8,
    NSOpenGLPFAAlphaSize                = 11,
    NSOpenGLPFADepthSize                = 12,
    NSOpenGLPFAStencilSize              = 13,
    NSOpenGLPFAAccumSize                = 14,
    NSOpenGLPFAMinimumPolicy            = 51,
    NSOpenGLPFAMaximumPolicy            = 52,
    NSOpenGLPFAOffScreen                = 53,
    NSOpenGLPFAFullScreen               = 54,
    NSOpenGLPFASampleBuffers            = 55,
    NSOpenGLPFASamples                  = 56,
    NSOpenGLPFAAuxDepthStencil          = 57,
    NSOpenGLPFAColorFloat               = 58,
    NSOpenGLPFAMultisample              = 59,
    NSOpenGLPFASupersample              = 60,
    NSOpenGLPFASampleAlpha              = 61,
    NSOpenGLPFARendererID               = 70,
    NSOpenGLPFASingleRenderer           = 71,
    NSOpenGLPFANoRecovery               = 72,
    NSOpenGLPFAAccelerated              = 73,
    NSOpenGLPFAClosestPolicy            = 74,
    NSOpenGLPFARobust                   = 75,
    NSOpenGLPFABackingStore             = 76,
    NSOpenGLPFAMPSafe                   = 78,
    NSOpenGLPFAWindow                   = 80,
    NSOpenGLPFAMultiScreen              = 81,
    NSOpenGLPFACompliant                = 83,
    NSOpenGLPFAScreenMask               = 84,
    NSOpenGLPFAPixelBuffer              = 90,
    NSOpenGLPFARemotePixelBuffer        = 91,
    NSOpenGLPFAAllowOfflineRenderers    = 96,
    NSOpenGLPFAAcceleratedCompute       = 97,
    NSOpenGLPFAOpenGLProfile            = 99,
    NSOpenGLPFAVirtualScreenCount       = 128,
}

#[repr(u64)]
pub enum NSEventType {
    NSLeftMouseDown         = 1,
    NSLeftMouseUp           = 2,
    NSRightMouseDown        = 3,
    NSRightMouseUp          = 4,
    NSMouseMoved            = 5,
    NSLeftMouseDragged      = 6,
    NSRightMouseDragged     = 7,
    NSMouseEntered          = 8,
    NSMouseExited           = 9,
    NSKeyDown               = 10,
    NSKeyUp                 = 11,
    NSFlagsChanged          = 12,
    NSAppKitDefined         = 13,
    NSSystemDefined         = 14,
    NSApplicationDefined    = 15,
    NSPeriodic              = 16,
    NSCursorUpdate          = 17,
    NSScrollWheel           = 22,
    NSTabletPoint           = 23,
    NSTabletProximity       = 24,
    NSOtherMouseDown        = 25,
    NSOtherMouseUp          = 26,
    NSOtherMouseDragged     = 27,
    NSEventTypeGesture      = 29,
    NSEventTypeMagnify      = 30,
    NSEventTypeSwipe        = 31,
    NSEventTypeRotate       = 18,
    NSEventTypeBeginGesture = 19,
    NSEventTypeEndGesture   = 20,
}
#[repr(u64)]
pub enum NSEventMask {
    NSLeftMouseDownMask         = 1 << NSLeftMouseDown as uint,
    NSLeftMouseUpMask           = 1 << NSLeftMouseUp as uint,
    NSRightMouseDownMask        = 1 << NSRightMouseDown as uint,
    NSRightMouseUpMask          = 1 << NSRightMouseUp as uint,
    NSMouseMovedMask            = 1 << NSMouseMoved as uint,
    NSLeftMouseDraggedMask      = 1 << NSLeftMouseDragged as uint,
    NSRightMouseDraggedMask     = 1 << NSRightMouseDragged as uint,
    NSMouseEnteredMask          = 1 << NSMouseEntered as uint,
    NSMouseExitedMask           = 1 << NSMouseExited as uint,
    NSKeyDownMask               = 1 << NSKeyDown as uint,
    NSKeyUpMask                 = 1 << NSKeyUp as uint,
    NSFlagsChangedMask          = 1 << NSFlagsChanged as uint,
    NSAppKitDefinedMask         = 1 << NSAppKitDefined as uint,
    NSSystemDefinedMask         = 1 << NSSystemDefined as uint,
    NSAPplicationDefinedMask    = 1 << NSApplicationDefined as uint,
    NSPeriodicMask              = 1 << NSPeriodic as uint,
    NSCursorUpdateMask          = 1 << NSCursorUpdate as uint,
    NSScrollWheelMask           = 1 << NSScrollWheel as uint,
    NSTabletPointMask           = 1 << NSTabletPoint as uint,
    NSTabletProximityMask       = 1 << NSTabletProximity as uint,
    NSOtherMouseDownMask        = 1 << NSOtherMouseDown as uint,
    NSOtherMouseUpMask          = 1 << NSOtherMouseUp as uint,
    NSOtherMouseDraggedMask     = 1 << NSOtherMouseDragged as uint,
    NSEventMaskgesture          = 1 << NSEventTypeGesture as uint,
    NSEventMaskSwipe            = 1 << NSEventTypeSwipe as uint,
    NSEventMaskRotate           = 1 << NSEventTypeRotate as uint,
    NSEventMaskBeginGesture     = 1 << NSEventTypeBeginGesture as uint,
    NSEventMaskEndGesture       = 1 << NSEventTypeEndGesture as uint,
    NSAnyEventMask              = 0xffffffff,
}

pub trait NSAutoreleasePool {
    unsafe fn new(_: Self) -> id {
        "NSAutoreleasePool".send("new", ())
    }

    unsafe fn autorelease(self) -> Self;
}

impl NSAutoreleasePool for id {
    unsafe fn autorelease(self) -> id {
        self.send("autorelease", ())
    }
}

pub trait NSProcessInfo {
    unsafe fn processInfo(_: Self) -> id {
        "NSProcessInfo".send("processInfo", ())
    }

    unsafe fn processName(self) -> id;
}

impl NSProcessInfo for id {
    unsafe fn processName(self) -> id {
        self.send("processName", ())
    }
}

pub trait NSApplication {
    unsafe fn sharedApplication(_: Self) -> id {
        "NSApplication".send("sharedApplication", ())
    }

    unsafe fn setActivationPolicy_(self, policy: NSApplicationActivationPolicy) -> bool;
    unsafe fn setMainMenu_(self, menu: id);
    unsafe fn activateIgnoringOtherApps_(self, ignore: bool);
    unsafe fn run(self);
    unsafe fn finishLaunching(self);
    unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(self,
                                                              mask: NSUInteger,
                                                              expiration: id,
                                                              in_mode: id,
                                                              dequeue: bool) -> id;
    unsafe fn sendEvent_(self, an_event: id);
}

impl NSApplication for id {
    unsafe fn setActivationPolicy_(self, policy: NSApplicationActivationPolicy) -> bool {
        self.send_bool("setActivationPolicy:", policy as NSInteger)
    }

    unsafe fn setMainMenu_(self, menu: id) {
        self.send_void("setMainMenu:", menu)
    }

    unsafe fn activateIgnoringOtherApps_(self, ignore: bool) {
        self.send_void("activateIgnoringOtherApps:", ignore);
    }

    unsafe fn run(self) {
        self.send_void("run", ());
    }

    unsafe fn finishLaunching(self) {
        self.send_void("finishLaunching", ())
    }

    unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(self,
                                                              mask: NSUInteger,
                                                              expiration: id,
                                                              in_mode: id,
                                                              dequeue: bool) -> id {
        self.send("nextEventMatchingMask:untilDate:inMode:dequeue:",
                  (mask, expiration, in_mode, dequeue))
    }

    unsafe fn sendEvent_(self, an_event: id) {
        self.send_void("sendEvent:", an_event)
    }
}

pub trait NSMenu {
    unsafe fn new(_: Self) -> id {
        "NSMenu".send("new", ())
    }

    unsafe fn addItem_(self, menu_item: id);
}

impl NSMenu for id {
    unsafe fn addItem_(self, menu_item: id) {
        self.send_void("addItem:", menu_item)
    }
}

pub trait NSMenuItem {
    unsafe fn alloc(_: Self) -> id {
        "NSMenuItem".send("alloc", ())
    }

    unsafe fn new(_: Self) -> id {
        "NSMenuItem".send("new", ())
    }

    unsafe fn initWithTitle_action_keyEquivalent_(self, title: id, action: SEL, key: id) -> id;
    unsafe fn setSubmenu_(self, submenu: id);
}

impl NSMenuItem for id {
    unsafe fn initWithTitle_action_keyEquivalent_(self, title: id, action: SEL, key: id) -> id {
        self.send("initWithTitle:action:keyEquivalent:", (title, action, key))
    }

    unsafe fn setSubmenu_(self, submenu: id) {
        self.send_void("setSubmenu:", submenu)
    }
}

pub trait NSWindow {
    unsafe fn alloc(_: Self) -> id {
        "NSWindow".send("alloc", ())
    }

    unsafe fn initWithContentRect_styleMask_backing_defer_(self,
                                                           rect: NSRect,
                                                           style: NSUInteger,
                                                           backing: NSBackingStoreType,
                                                           defer: bool) -> id;
    unsafe fn cascadeTopLeftFromPoint_(self, top_left: NSPoint) -> NSPoint;
    unsafe fn setTitle_(self, title: id);
    unsafe fn makeKeyAndOrderFront_(self, sender: id);
    unsafe fn center(self);
    unsafe fn setContentView(self, view: id);
}

impl NSWindow for id {
    unsafe fn initWithContentRect_styleMask_backing_defer_(self,
                                                           rect: NSRect,
                                                           style: NSUInteger,
                                                           backing: NSBackingStoreType,
                                                           defer: bool) -> id {
        self.send("initWithContentRect:styleMask:backing:defer:",
                  (rect, style, backing as NSUInteger, defer))
    }

    unsafe fn cascadeTopLeftFromPoint_(self, top_left: NSPoint) -> NSPoint {
        self.send_point("cascadeTopLeftFromPoint:", top_left)
    }

    unsafe fn setTitle_(self, title: id) {
        self.send_void("setTitle:", title);
    }

    unsafe fn makeKeyAndOrderFront_(self, sender: id) {
        self.send_void("makeKeyAndOrderFront:", sender)
    }

    unsafe fn center(self) {
        self.send_void("center", ())
    }

    unsafe fn setContentView(self, view: id) {
        self.send_void("setContentView:", view)
    }
}

pub trait NSString {
    unsafe fn alloc(_: Self) -> id {
        "NSString".send("alloc", ())
    }

    unsafe fn initWithUTF8String_(self, c_string: *const u8) -> id;
    unsafe fn stringByAppendingString_(self, other: id) -> id;
    unsafe fn init_str(self, string: &str) -> Self;
}

impl NSString for id {
    unsafe fn initWithUTF8String_(self, c_string: *const u8) -> id {
        self.send("initWithUTF8String:", c_string as id)
    }

    unsafe fn stringByAppendingString_(self, other: id) -> id {
        self.send("stringByAppendingString:", other)
    }

    unsafe fn init_str(self, string: &str) -> id {
        self.initWithUTF8String_(string.as_ptr())
    }
}

pub trait NSView {
    unsafe fn alloc(_: Self) -> id {
        "NSView".send("alloc", ())
    }

    unsafe fn init(self) -> id;
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id;
    unsafe fn display_(self);
    unsafe fn setWantsBestResolutionOpenGLSurface_(self, flag: bool);
}

impl NSView for id {
    unsafe fn init(self) -> id {
        self.send("init", ())
    }

    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        self.send("initWithFrame:", frameRect)
    }

    unsafe fn display_(self) {
        self.send_void("display", ())
    }

    unsafe fn setWantsBestResolutionOpenGLSurface_(self, flag: bool) {
        self.send_void("setWantsBestResolutionOpenGLSurface:", flag)
    }
}

pub trait NSOpenGLView {
    unsafe fn alloc(_: Self) -> id {
        "NSOpenGLView".send("alloc", ())
    }

    unsafe fn initWithFrame_pixelFormat_(self, frameRect: NSRect, format: id) -> id;
    unsafe fn display_(self);
}

impl NSOpenGLView for id {
    unsafe fn initWithFrame_pixelFormat_(self,  frameRect: NSRect, format: id) -> id {
        self.send("initWithFrame:pixelFormat:", (frameRect, format))
    }

    unsafe fn display_(self) {
        self.send_void("display", ())
    }
}

pub trait NSOpenGLPixelFormat {
    unsafe fn alloc(_: Self) -> id {
        "NSOpenGLPixelFormat".send("alloc", ())
    }

    unsafe fn initWithAttributes_(self, attributes: &[uint]) -> id;
}

impl NSOpenGLPixelFormat for id {
    unsafe fn initWithAttributes_(self, attributes: &[uint]) -> id {
        self.send("initWithAttributes:", attributes)
    }
}

pub trait NSOpenGLContext {
    unsafe fn alloc(_: Self) -> id {
        "NSOpenGLContext".send("alloc", ())
    }

    unsafe fn initWithFormat_shareContext_(self, format: id, shareContext: id) -> id;
    unsafe fn setView_(self, view: id);
    unsafe fn makeCurrentContext(self);
    unsafe fn flushBuffer(self);
}

impl NSOpenGLContext for id {
    unsafe fn initWithFormat_shareContext_(self, format: id, shareContext: id) -> id {
        self.send("initWithFormat:shareContext:", (format, shareContext))
    }

    unsafe fn setView_(self, view: id) {
        self.send_void("setView:", view)
    }

    unsafe fn makeCurrentContext(self) {
        self.send_void("makeCurrentContext", ())
    }

    unsafe fn flushBuffer(self) {
        self.send_void("flushBuffer", ())
    }
}
