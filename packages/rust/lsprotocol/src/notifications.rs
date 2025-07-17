// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// ****** THIS IS A GENERATED FILE, DO NOT EDIT. ******
// Steps to generate:
// 1. Checkout https://github.com/microsoft/lsprotocol
// 2. Install nox: `python -m pip install nox`
// 3. Run command: `python -m nox --session build_lsp`

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

use crate::*;

pub trait Notification {
    type Params: serde::de::DeserializeOwned + serde::Serialize + Send + Sync + 'static;
    const METHOD: &'static str;
}

/// The `workspace/didChangeWorkspaceFolders` notification is sent from the client to the server when the workspace
/// folder configuration changes.
pub struct WorkspaceDidChangeWorkspaceFoldersNotification;

impl Notification for WorkspaceDidChangeWorkspaceFoldersNotification {
    type Params = DidChangeWorkspaceFoldersParams;
    const METHOD: &'static str = "workspace/didChangeWorkspaceFolders";
}

/// The `window/workDoneProgress/cancel` notification is sent from  the client to the server to cancel a progress
/// initiated on the server side.
pub struct WindowWorkDoneProgressCancelNotification;

impl Notification for WindowWorkDoneProgressCancelNotification {
    type Params = WorkDoneProgressCancelParams;
    const METHOD: &'static str = "window/workDoneProgress/cancel";
}

/// The did create files notification is sent from the client to the server when
/// files were created from within the client.
///
/// @since 3.16.0
pub struct WorkspaceDidCreateFilesNotification;

impl Notification for WorkspaceDidCreateFilesNotification {
    type Params = CreateFilesParams;
    const METHOD: &'static str = "workspace/didCreateFiles";
}

/// The did rename files notification is sent from the client to the server when
/// files were renamed from within the client.
///
/// @since 3.16.0
pub struct WorkspaceDidRenameFilesNotification;

impl Notification for WorkspaceDidRenameFilesNotification {
    type Params = RenameFilesParams;
    const METHOD: &'static str = "workspace/didRenameFiles";
}

/// The will delete files request is sent from the client to the server before files are actually
/// deleted as long as the deletion is triggered from within the client.
///
/// @since 3.16.0
pub struct WorkspaceDidDeleteFilesNotification;

impl Notification for WorkspaceDidDeleteFilesNotification {
    type Params = DeleteFilesParams;
    const METHOD: &'static str = "workspace/didDeleteFiles";
}

/// A notification sent when a notebook opens.
///
/// @since 3.17.0
pub struct NotebookDocumentDidOpenNotification;

impl Notification for NotebookDocumentDidOpenNotification {
    type Params = DidOpenNotebookDocumentParams;
    const METHOD: &'static str = "notebookDocument/didOpen";
}

pub struct NotebookDocumentDidChangeNotification;

impl Notification for NotebookDocumentDidChangeNotification {
    type Params = DidChangeNotebookDocumentParams;
    const METHOD: &'static str = "notebookDocument/didChange";
}

/// A notification sent when a notebook document is saved.
///
/// @since 3.17.0
pub struct NotebookDocumentDidSaveNotification;

impl Notification for NotebookDocumentDidSaveNotification {
    type Params = DidSaveNotebookDocumentParams;
    const METHOD: &'static str = "notebookDocument/didSave";
}

/// A notification sent when a notebook closes.
///
/// @since 3.17.0
pub struct NotebookDocumentDidCloseNotification;

impl Notification for NotebookDocumentDidCloseNotification {
    type Params = DidCloseNotebookDocumentParams;
    const METHOD: &'static str = "notebookDocument/didClose";
}

/// The initialized notification is sent from the client to the
/// server after the client is fully initialized and the server
/// is allowed to send requests from the server to the client.
pub struct InitializedNotification;

impl Notification for InitializedNotification {
    type Params = InitializedParams;
    const METHOD: &'static str = "initialized";
}

/// The exit event is sent from the client to the server to
/// ask the server to exit its process.
pub struct ExitNotification;

impl Notification for ExitNotification {
    type Params = ();
    const METHOD: &'static str = "exit";
}

/// The configuration change notification is sent from the client to the server
/// when the client's configuration has changed. The notification contains
/// the changed configuration as defined by the language client.
pub struct WorkspaceDidChangeConfigurationNotification;

impl Notification for WorkspaceDidChangeConfigurationNotification {
    type Params = DidChangeConfigurationParams;
    const METHOD: &'static str = "workspace/didChangeConfiguration";
}

/// The show message notification is sent from a server to a client to ask
/// the client to display a particular message in the user interface.
pub struct WindowShowMessageNotification;

impl Notification for WindowShowMessageNotification {
    type Params = ShowMessageParams;
    const METHOD: &'static str = "window/showMessage";
}

/// The log message notification is sent from the server to the client to ask
/// the client to log a particular message.
pub struct WindowLogMessageNotification;

impl Notification for WindowLogMessageNotification {
    type Params = LogMessageParams;
    const METHOD: &'static str = "window/logMessage";
}

/// The telemetry event notification is sent from the server to the client to ask
/// the client to log telemetry data.
pub struct TelemetryEventNotification;

impl Notification for TelemetryEventNotification {
    type Params = LSPAny;
    const METHOD: &'static str = "telemetry/event";
}

/// The document open notification is sent from the client to the server to signal
/// newly opened text documents. The document's truth is now managed by the client
/// and the server must not try to read the document's truth using the document's
/// uri. Open in this sense means it is managed by the client. It doesn't necessarily
/// mean that its content is presented in an editor. An open notification must not
/// be sent more than once without a corresponding close notification send before.
/// This means open and close notification must be balanced and the max open count
/// is one.
pub struct TextDocumentDidOpenNotification;

impl Notification for TextDocumentDidOpenNotification {
    type Params = DidOpenTextDocumentParams;
    const METHOD: &'static str = "textDocument/didOpen";
}

/// The document change notification is sent from the client to the server to signal
/// changes to a text document.
pub struct TextDocumentDidChangeNotification;

impl Notification for TextDocumentDidChangeNotification {
    type Params = DidChangeTextDocumentParams;
    const METHOD: &'static str = "textDocument/didChange";
}

/// The document close notification is sent from the client to the server when
/// the document got closed in the client. The document's truth now exists where
/// the document's uri points to (e.g. if the document's uri is a file uri the
/// truth now exists on disk). As with the open notification the close notification
/// is about managing the document's content. Receiving a close notification
/// doesn't mean that the document was open in an editor before. A close
/// notification requires a previous open notification to be sent.
pub struct TextDocumentDidCloseNotification;

impl Notification for TextDocumentDidCloseNotification {
    type Params = DidCloseTextDocumentParams;
    const METHOD: &'static str = "textDocument/didClose";
}

/// The document save notification is sent from the client to the server when
/// the document got saved in the client.
pub struct TextDocumentDidSaveNotification;

impl Notification for TextDocumentDidSaveNotification {
    type Params = DidSaveTextDocumentParams;
    const METHOD: &'static str = "textDocument/didSave";
}

/// A document will save notification is sent from the client to the server before
/// the document is actually saved.
pub struct TextDocumentWillSaveNotification;

impl Notification for TextDocumentWillSaveNotification {
    type Params = WillSaveTextDocumentParams;
    const METHOD: &'static str = "textDocument/willSave";
}

/// The watched files notification is sent from the client to the server when
/// the client detects changes to file watched by the language client.
pub struct WorkspaceDidChangeWatchedFilesNotification;

impl Notification for WorkspaceDidChangeWatchedFilesNotification {
    type Params = DidChangeWatchedFilesParams;
    const METHOD: &'static str = "workspace/didChangeWatchedFiles";
}

/// Diagnostics notification are sent from the server to the client to signal
/// results of validation runs.
pub struct TextDocumentPublishDiagnosticsNotification;

impl Notification for TextDocumentPublishDiagnosticsNotification {
    type Params = PublishDiagnosticsParams;
    const METHOD: &'static str = "textDocument/publishDiagnostics";
}

pub struct SetTraceNotification;

impl Notification for SetTraceNotification {
    type Params = SetTraceParams;
    const METHOD: &'static str = "setTrace";
}

pub struct LogTraceNotification;

impl Notification for LogTraceNotification {
    type Params = LogTraceParams;
    const METHOD: &'static str = "logTrace";
}

pub struct CancelRequestNotification;

impl Notification for CancelRequestNotification {
    type Params = CancelParams;
    const METHOD: &'static str = "cancelRequest";
}

pub struct ProgressNotification;

impl Notification for ProgressNotification {
    type Params = ProgressParams;
    const METHOD: &'static str = "progress";
}
