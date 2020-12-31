/// This file contains data structures for internal events that components might be interested in receiving
/// notifications about. Each component object can register for UI events (window messages) which it will be
/// responsible for handling.
///

pub enum InternalEvent {
    FileOpenDialogOpened(),
    FileOpenDialogSucceeded(),
    FileOpenDialogFailed(),

    FileOpened(),
    FileClosed(),

    FindOpened(),
    FindClosed(),
    FindResultAvailable(),

    DataSelected(),
    DataCopied(),

    PreferencesDialogOpened(),
    PreferencesDialogSucceeded(),
    PreferencesDialogCancelled(),

    PreferencesLoaded(),
    PreferencesChanged(),
    PreferencesSaved(),

    TabOpened(),
    TabClosed(),
    TabChanged(),

    AboutDialogOpened(),
    AboutDialogClosed(),

    WindowPositionChanged(),
    WindowSizeChanged(),
    WindowMinimized(),
    WindowClosing(),
}