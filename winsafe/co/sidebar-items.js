initSidebarItems({"struct":[["ACCELF","`ACCELL` `fVirt` (`u8`)."],["ACCESS_RIGHTS","`RegOpenKeyEx` `samDesired` (`u32`). Originally has no prefix."],["ADRF","`NMTVASYNCDRAW` `dwRetFlags` (`u32`). Don't seem to be defined anywhere, unconfirmed values."],["APPCOMMAND","`WM_APPCOMMAND` commands (`u16`)."],["BCN","Button control `WM_NOTIFY` notifications (`i32`), convertible to `NM`."],["BCSIF","`BUTTON_SPLITINFO` `mask` (`u32`)."],["BCSS","`BUTTON_SPLITINFO` `uSplitStyle` (`u32`)."],["BI","`BITMAPINFOHEADER` `biCompression` (`u32`)."],["BIA","`BUTTON_IMAGELIST` `uAlign` (`u32`). Originally has `BUTTON_IMAGELIST_ALIGN_` prefix."],["BKMODE","`SetBkMode` `mode` (`i32`)."],["BM","Button control messages (`u32`), convertible to `WM`."],["BN","Button control `WM_COMMAND` notifications (`u16`), convertible to `CMD`."],["BS","Button control styles (`u32`), convertible to `WS`."],["BST","`BM_GETCHECK` return value (`u32`)."],["CB","Combo box control messages (`u32`), convertible to `WM`."],["CBES_EX","Extended combo box styles (`u32`), convertible to `WS_EX`."],["CBN","Combo box control `WM_COMMAND` notifications (`u16`), convertible to `CMD`."],["CBS","Combo box control styles (`u32`), convertible to `WS`."],["CCM","Generic common controls messages (`u32`), convertible to `WM`."],["CDDS","`NMCUSTOMDRAW` `dwDrawStage` (`u32`)."],["CDIS","`NMCUSTOMDRAW` `uItemState` (`u32`)."],["CDRF","`NM_CUSTOMDRAW` return value (`u32`)."],["CHARSET","`LOGFONT` `lfCharset` (`u8`)."],["CLIP","`LOGFONT` `lfClipPrecision` (`u8`)."],["CLR","`IMAGELISTDRAWPARAMS` `rgbFg` (`u32`)."],["CLSCTX","`CLSCTX` enumeration (`u32`)."],["CMD","`WM_COMMAND` notification codes (`u16`)."],["COINIT","`CoInitializeEx` `dwCoInit` (`u32`)."],["COLOR","System colors (`u32`)."],["CS","Window class `styles` (`u32`)."],["DDL","`CB_DIR` attributes (`u16`)."],["DLGID","Dialog built-in IDs (`u16`). These are also returned from `MessageBox`."],["DTM","Date and time picker control messages (`u32`), convertible to `WM`."],["DTN","Date and time picker control `WM_NOTIFY` notifications (`i32`), convertible to `NM`."],["DTS","Date and time picker control styles (`u32`), convertible to `WS`."],["EMF","`NMLVEMPTYMARKUP` `dwFlags` (`u32`)."],["EN","Edit control `WM_COMMAND` notifications (`u16`), convertible to `CMD`."],["ENDSESSION","`WM_ENDSESSION` event (`u32`)."],["ERROR","A Windows system error code (`u32`) retrieved by `GetLastError` function, or an `HRESULT`."],["ES","Edit control styles (`u32`), convertible to `WS`."],["FAPPCOMMAND","`WM_APPCOMMAND` input event (`u16`)."],["FF","`LOGFONT` `lfPitchAndFamily` (`u8`), used with `PITCH`."],["FORMAT_MESSAGE","`FormatMessage` `dwFlags` (`u32`)."],["FW","`LOGFONT` `lfWeight` (`u32`)."],["GA","`GetAncestor` `gaFlags` (`u32`)."],["GCLP","`GetClassLongPtr` `nIndex` (`i32`). Originally has prefixes `GCW` and `GCL` also."],["GDC","`GetDeviceCaps` `index` (`i32`). Originally has no prefix."],["GDT","`NMDATETIMECHANGE` and `NMDATETIMESTRING` `dwFlags` (`u32`)."],["GDTR","`DTM_GETRANGE` return value (`u32`)."],["GW","`GetWindow` `uCmd` (`u32`)."],["GWLP","`GetWindowLongPtr` and `SetWindowLongPtr` `nIndex` (`i32`). Originally has prefix `GWL` also."],["GWL_C","`WM_STYLECHANGED` and `WM_STYLECHANGING` change (`i8`). Originally has `GWL` prefix."],["HDM","Header control messages (`u32`), convertible to `WM`."],["HDN","Header control `WM_NOTIFY` notifications (`i32`), convertible to `NM`."],["HDS","Header control styles (`u32`), convertible to `WS`."],["HICF","NMBCHOTITEM `dwFlags` (`u32`)."],["HWND_PLACE","`SetWindowPos` `hWndInsertAfter` (`isize`)."],["ICON_SZ","`WM_SETICON` icon size (`u8`). Originally has `ICON` prefix."],["IDC","`LoadCursor` `lpCursorName` (`usize`)."],["IDI","`LoadIcon` `lpIconName` (`usize`)."],["ILC","`ImageList_Create` `flags` (`u32`)."],["ILD","`IMAGELISTDRAWFLAGS` enumeration (`u32`)."],["ILS","`IMAGELISTSTATEFLAGS` enumeration (`u32`)."],["IMAGE_TYPE","`BM_GETIMAGE` `img_type` (`u8`). Originally has `IMAGE` prefix."],["IPM","IP address control messages (`u32`), convertible to `WM`."],["IPN","IP address control `WM_NOTIFY` notifications (`i32`), convertible to `NM`."],["KEY","`RegOpenKeyEx` `samDesired` (`u32`)."],["LANG","`FormatMessage` `dwLanguageId` (`u16`), used with `SUBLANG`."],["LB","List box control messages (`u32`), convertible to `WM`."],["LBN","List box control `WM_COMMAND` notifications (`u16`), convertible to `CMD`."],["LBS","List box control styles (`u32`), convertible to `WS`."],["LIF","`LITEM` `mask` (`u32`)."],["LIS","`LITEM` `state` (`u32`)."],["LR","`LoadImageBitmap`, `LoadImageCursor` and `LoadImageIcon` `fuLoad`."],["LSFW","`LockSetForegroundWindow` `uLockCode` (`u32`)."],["LVA","`LVM_ARRANGE` arrangement (`u16`)."],["LVCF","`LVCOLUMN` `mask` (`u32`)."],["LVCFMT_C","`LVCOLUMN` `mask` (`i32`)."],["LVCFMT_I","`LVITEM` `piColFmt` (`i32`)."],["LVFI","`LVFINDINFO` `flags` (`u32`)."],["LVGIT","`NMLVGETINFOTIP` `dwFlags` (`u32`)."],["LVIF","`LVITEM` `mask` (`u32`)."],["LVIS","ListView item states (`u32`)."],["LVI_GROUPID","`LVITEM` `iGroupId` (`i32`)."],["LVKF","`NMITEMACTIVATE` `uKeyFlags` (`u32`)."],["LVM","List view control messages (`u32`), convertible to `WM`."],["LVN","List view control `WM_NOTIFY` notifications (`i32`), convertible to `NM`."],["LVNI","`LVM_GETNEXTITEM` relationship (`u32`)."],["LVS","List view control styles (`u32`), convertible to `WS`."],["LVS_EX","Extended list view control styles (`u32`), convertible to `WS_EX`."],["LV_VIEW","ListView views (`u32`)."],["LWS","SysLink control styles (`u32`), convertible to `WS`."],["MB","`MessageBox` `uType` (`u32`)."],["MCS","Month calendar control styles (`u32`), convertible to `WS`."],["MCSC","`DTM_GETMCCOLOR` color (`u8`)."],["MF","`AppendMenu` and `InsertMenu` `uFlags` (`u32`)."],["MFS","`MENUITEMINFO` `fState` (`u32`)."],["MFT","`MENUITEMINFO` `fType` (`u32`)."],["MIIM","`MENUITEMINFO` `fMask` (`u32`)."],["MIM","`MENUINFO` `fMask` (`u32`)."],["MK","`WM_LBUTTONDOWN` (and similar) virtual keys (`u16`)."],["MNS","`MENUINFO` `dwStyle` (`u32`)."],["MSGF","`WM_ENTERIDLE` reason (`u8`)."],["NM","`WM_NOTIFY` notification codes (`i32`)."],["OCR","`SetSystemCursor` `id` (`u32`)."],["OUT_PRECIS","`LOGFONT` `lfOutPrecision` (`u8`)."],["PBS","Progress bar control styles (`u32`), convertible to `WS`."],["PITCH","`LOGFONT` `lfPitchAndFamily` (`u8`), used with `FF`."],["PM","`PeekMessage` `wRemoveMsg` (`u32`)."],["QS","`GetQueueStatus` `flags` (`u32`)."],["QUALITY","`LOGFONT` `lfQuality` (`u8`)."],["RBS","Rebar control styles (`u32`), convertible to `WS`."],["RDW","`RedrawWindow` `flags` (`u32`)."],["REG","Registry value types (`u32`)."],["REGION","`GetUpdateRgn`, `GetWindowRgn` and `SelectObjectRgn` return value (`i32`)."],["REG_OPTION","`RegOpenKeyEx` `uOptions` (`u32`)."],["ROP","`IMAGELISTDRAWPARAMS` `dwRop` (`u32`)."],["RRF","`RegGetValue` `dwFlags` (`u32`)."],["SB","Status bar control messages (`u32`). Convertible to `WM`."],["SBARS","Status bar control styles (`u32`), convertible to `WS`."],["SBN","Status bar control `WM_NOTIFY` notifications (`i32`), convertible to `NM`."],["SBT","`SB_GETTEXT`, `SB_GETTEXTLENGTH` and `SB_SETTEXT` drawing operation (`u16`)."],["SC","`WM_SYSCOMMAND` type of system command requested (`u32`)."],["SIZE_R","`WM_SIZE` request (`u8`)."],["SM","`GetSystemMetrics` `nIndex` (`i32`)."],["SPI","`SystemParametersInfo` `uiAction` (`u32`)."],["SPIF","`SystemParametersInfo` `fWinIni` (`u32`)."],["SS","Label control styles (`u32`), convertible to `WS`."],["STANDARD_RIGHTS","`RegOpenKeyEx` `samDesired` (`u32`)."],["STAP","`GetThemeAppProperties` return value (`u32`)."],["STATE_SYSTEM","`DATETIMEPICKERINFO` `stateCheck` and `stateButton` (`u32`)."],["STM","Static control messages (`u32`), convertible to `WM`."],["STN","Static control `WM_COMMAND` notifications (`u16`), convertible to `CMD`."],["SUBLANG","`FormatMessage` `dwLanguageId` (`u16`), used with `LANG`."],["SW","`ShowWindow` `nCmdShow` (`i32`)."],["SWP","`SetWindowPos` `uFlags` (`u32`)."],["SW_S","`WM_SHOWWINDOW` status (`u8`). Originally has `SW` prefix."],["TBS","Trackbar control styles (`u32`), convertible to `WS`."],["TCS","Tab control styles (`u32`), convertible to `WS`."],["TCS_EX","Extended tab control styles (`u32`), convertible to `WS_EX`."],["TME","`TrackMouseEvent` `dwFlags` (`u32`)."],["TPM","`TrackPopupMenu` `uFlags` (`u32`)."],["TVM","Tree view control messages (`u32`), convertible to `WM`."],["TVN","Tree view control `WM_NOTIFY` notifications (`i32`), convertible to `NM`."],["TVS","Tree view control styles (`u32`), convertible to `WS`."],["TVS_EX","Extended tree view control styles (`u32`), convertible to `WS_EX`."],["UDS","Up-down control styles (`u32`), convertible to `WS`."],["VER_COND","`VerSetConditionMask` `Condition` (`u8`)."],["VER_MASK","`VerSetConditionMask` `TypeMask` (`u32`)."],["VER_NT","`OSVERSIONINFOEX` `wProductType` (`u8`)."],["VER_PLATFORM","`OSVERSIONINFOEX` `dwPlatformId` (`u32`)."],["VER_SUITE","`OSVERSIONINFOEX` `wSuiteMask` (`u16`)."],["VK","Virtual key codes (`u16`)."],["VK_DIR","`LVFINDINFO` `vkDirection` (`u16`)."],["VS_PART","System visual style part (`i32`)."],["VS_STATE","System visual style state (`i32`)."],["WA","`WM_ACTIVATE` activation state (`u16`)."],["WDA","`GetWindowDisplayAffinity` and `SetWindowDisplayAffinity` `dwAffinity` (`u32`)."],["WH","`SetWindowsHookEx` `idHook` (`i32`)."],["WIN32","`_WIN32` version definitions (`u16`)."],["WM","Window message codes (`u32`)."],["WMSZ","`WM_SIZING` window edge (`u8`)."],["WPF","`WINDOWPLACEMENT` `flags` (`u32`)."],["WS","Window styles (`u32`)."],["WS_EX","Extended window styles (`u32`)."],["WVR","`WM_NCCALCSIZE` return flags (`u32`)."]]});