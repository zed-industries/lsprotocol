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
/// A request to resolve the implementation locations of a symbol at a given text
/// document position. The request's parameter is of type [TextDocumentPositionParams]
/// the response is of type [Definition] or a Thenable that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImplementationRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: ImplementationParams,
}

/// A request to resolve the type definition locations of a symbol at a given text
/// document position. The request's parameter is of type [TextDocumentPositionParams]
/// the response is of type [Definition] or a Thenable that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypeDefinitionRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: TypeDefinitionParams,
}

/// The `workspace/workspaceFolders` is sent from the server to the client to fetch the open workspace folders.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkspaceFoldersRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: Option<()>,
}

/// The 'workspace/configuration' request is sent from the server to the client to fetch a certain
/// configuration setting.
///
/// This pull model replaces the old push model were the client signaled configuration change via an
/// event. If the server still needs to react to configuration changes (since the server caches the
/// result of `workspace/configuration` requests) the server should register for an empty configuration
/// change event and empty the cache if such an event is received.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfigurationRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: ConfigurationParams,
}

/// A request to list all color symbols found in a given text document. The request's
/// parameter is of type [DocumentColorParams] the
/// response is of type {@link ColorInformation ColorInformation[]} or a Thenable
/// that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentColorRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DocumentColorParams,
}

/// A request to list all presentation for a color. The request's
/// parameter is of type [ColorPresentationParams] the
/// response is of type {@link ColorInformation ColorInformation[]} or a Thenable
/// that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ColorPresentationRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: ColorPresentationParams,
}

/// A request to provide folding ranges in a document. The request's
/// parameter is of type [FoldingRangeParams], the
/// response is of type [FoldingRangeList] or a Thenable
/// that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FoldingRangeRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: FoldingRangeParams,
}

/// @since 3.18.0
/// @proposed
#[cfg(feature = "proposed")]
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FoldingRangeRefreshRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: Option<()>,
}

/// A request to resolve the type definition locations of a symbol at a given text
/// document position. The request's parameter is of type [TextDocumentPositionParams]
/// the response is of type [Declaration] or a typed array of [DeclarationLink]
/// or a Thenable that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeclarationRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DeclarationParams,
}

/// A request to provide selection ranges in a document. The request's
/// parameter is of type [SelectionRangeParams], the
/// response is of type {@link SelectionRange SelectionRange[]} or a Thenable
/// that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelectionRangeRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: SelectionRangeParams,
}

/// The `window/workDoneProgress/create` request is sent from the server to the client to initiate progress
/// reporting from the server.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkDoneProgressCreateRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: WorkDoneProgressCreateParams,
}

/// A request to result a `CallHierarchyItem` in a document at a given position.
/// Can be used as an input to an incoming or outgoing call hierarchy.
///
/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CallHierarchyPrepareRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: CallHierarchyPrepareParams,
}

/// A request to resolve the incoming calls for a given `CallHierarchyItem`.
///
/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CallHierarchyIncomingCallsRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: CallHierarchyIncomingCallsParams,
}

/// A request to resolve the outgoing calls for a given `CallHierarchyItem`.
///
/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CallHierarchyOutgoingCallsRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: CallHierarchyOutgoingCallsParams,
}

/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SemanticTokensRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: SemanticTokensParams,
}

/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SemanticTokensDeltaRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: SemanticTokensDeltaParams,
}

/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SemanticTokensRangeRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: SemanticTokensRangeParams,
}

/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SemanticTokensRefreshRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: Option<()>,
}

/// A request to show a document. This request might open an
/// external program depending on the value of the URI to open.
/// For example a request to open `https://code.visualstudio.com/`
/// will very likely open the URI in a WEB browser.
///
/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShowDocumentRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: ShowDocumentParams,
}

/// A request to provide ranges that can be edited together.
///
/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LinkedEditingRangeRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: LinkedEditingRangeParams,
}

/// The will create files request is sent from the client to the server before files are actually
/// created as long as the creation is triggered from within the client.
///
/// The request can return a `WorkspaceEdit` which will be applied to workspace before the
/// files are created. Hence the `WorkspaceEdit` can not manipulate the content of the file
/// to be created.
///
/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WillCreateFilesRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: CreateFilesParams,
}

/// The will rename files request is sent from the client to the server before files are actually
/// renamed as long as the rename is triggered from within the client.
///
/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WillRenameFilesRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: RenameFilesParams,
}

/// The did delete files notification is sent from the client to the server when
/// files were deleted from within the client.
///
/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WillDeleteFilesRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DeleteFilesParams,
}

/// A request to get the moniker of a symbol at a given text document position.
/// The request parameter is of type [TextDocumentPositionParams].
/// The response is of type {@link Moniker Moniker[]} or `null`.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MonikerRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: MonikerParams,
}

/// A request to result a `TypeHierarchyItem` in a document at a given position.
/// Can be used as an input to a subtypes or supertypes type hierarchy.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypeHierarchyPrepareRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: TypeHierarchyPrepareParams,
}

/// A request to resolve the supertypes for a given `TypeHierarchyItem`.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypeHierarchySupertypesRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: TypeHierarchySupertypesParams,
}

/// A request to resolve the subtypes for a given `TypeHierarchyItem`.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypeHierarchySubtypesRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: TypeHierarchySubtypesParams,
}

/// A request to provide inline values in a document. The request's parameter is of
/// type [InlineValueParams], the response is of type
/// {@link InlineValue InlineValue[]} or a Thenable that resolves to such.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InlineValueRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: InlineValueParams,
}

/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InlineValueRefreshRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: Option<()>,
}

/// A request to provide inlay hints in a document. The request's parameter is of
/// type [InlayHintsParams], the response is of type
/// {@link InlayHint InlayHint[]} or a Thenable that resolves to such.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InlayHintRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: InlayHintParams,
}

/// A request to resolve additional properties for an inlay hint.
/// The request's parameter is of type [InlayHint], the response is
/// of type [InlayHint] or a Thenable that resolves to such.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InlayHintResolveRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: InlayHint,
}

/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InlayHintRefreshRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: Option<()>,
}

/// The document diagnostic request definition.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentDiagnosticRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DocumentDiagnosticParams,
}

/// The workspace diagnostic request definition.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkspaceDiagnosticRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: WorkspaceDiagnosticParams,
}

/// The diagnostic refresh request definition.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagnosticRefreshRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: Option<()>,
}

/// A request to provide inline completions in a document. The request's parameter is of
/// type [InlineCompletionParams], the response is of type
/// {@link InlineCompletion InlineCompletion[]} or a Thenable that resolves to such.
///
/// @since 3.18.0
/// @proposed
#[cfg(feature = "proposed")]
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InlineCompletionRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: InlineCompletionParams,
}

/// The `workspace/textDocumentContent` request is sent from the client to the
/// server to request the content of a text document.
///
/// @since 3.18.0
/// @proposed
#[cfg(feature = "proposed")]
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TextDocumentContentRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: TextDocumentContentParams,
}

/// The `workspace/textDocumentContent` request is sent from the server to the client to refresh
/// the content of a specific text document.
///
/// @since 3.18.0
/// @proposed
#[cfg(feature = "proposed")]
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TextDocumentContentRefreshRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: TextDocumentContentRefreshParams,
}

/// The `client/registerCapability` request is sent from the server to the client to register a new capability
/// handler on the client side.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RegistrationRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: RegistrationParams,
}

/// The `client/unregisterCapability` request is sent from the server to the client to unregister a previously registered capability
/// handler on the client side.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnregistrationRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: UnregistrationParams,
}

/// The initialize request is sent from the client to the server.
/// It is sent once as the request after starting up the server.
/// The requests parameter is of type [InitializeParams]
/// the response if of type [InitializeResult] of a Thenable that
/// resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InitializeRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: InitializeParams,
}

/// A shutdown request is sent from the client to the server.
/// It is sent once when the client decides to shutdown the
/// server. The only notification that is sent after a shutdown request
/// is the exit event.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShutdownRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: Option<()>,
}

/// The show message request is sent from the server to the client to show a message
/// and a set of options actions to the user.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShowMessageRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: ShowMessageRequestParams,
}

/// A document will save request is sent from the client to the server before
/// the document is actually saved. The request can return an array of TextEdits
/// which will be applied to the text document before it is saved. Please note that
/// clients might drop results if computing the text edits took too long or if a
/// server constantly fails on this request. This is done to keep the save fast and
/// reliable.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WillSaveTextDocumentWaitUntilRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: WillSaveTextDocumentParams,
}

/// Request to request completion at a given text document position. The request's
/// parameter is of type [TextDocumentPosition] the response
/// is of type {@link CompletionItem CompletionItem[]} or [CompletionList]
/// or a Thenable that resolves to such.
///
/// The request can delay the computation of the [`detail`][`CompletionItem::detail`]
/// and [`documentation`][`CompletionItem::documentation`] properties to the `completionItem/resolve`
/// request. However, properties that are needed for the initial sorting and filtering, like `sortText`,
/// `filterText`, `insertText`, and `textEdit`, must not be changed during resolve.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompletionRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: CompletionParams,
}

/// Request to resolve additional information for a given completion item.The request's
/// parameter is of type [CompletionItem] the response
/// is of type [CompletionItem] or a Thenable that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompletionResolveRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: CompletionItem,
}

/// Request to request hover information at a given text document position. The request's
/// parameter is of type [TextDocumentPosition] the response is of
/// type [Hover] or a Thenable that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HoverRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: HoverParams,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SignatureHelpRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: SignatureHelpParams,
}

/// A request to resolve the definition location of a symbol at a given text
/// document position. The request's parameter is of type [TextDocumentPosition]
/// the response is of either type [Definition] or a typed array of
/// [DefinitionLink] or a Thenable that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DefinitionRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DefinitionParams,
}

/// A request to resolve project-wide references for the symbol denoted
/// by the given text document position. The request's parameter is of
/// type [ReferenceParams] the response is of type
/// {@link Location Location[]} or a Thenable that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReferencesRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: ReferenceParams,
}

/// Request to resolve a [DocumentHighlight] for a given
/// text document position. The request's parameter is of type [TextDocumentPosition]
/// the request response is an array of type [DocumentHighlight]
/// or a Thenable that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentHighlightRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DocumentHighlightParams,
}

/// A request to list all symbols found in a given text document. The request's
/// parameter is of type [TextDocumentIdentifier] the
/// response is of type {@link SymbolInformation SymbolInformation[]} or a Thenable
/// that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentSymbolRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DocumentSymbolParams,
}

/// A request to provide commands for the given text document and range.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CodeActionRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: CodeActionParams,
}

/// Request to resolve additional information for a given code action.The request's
/// parameter is of type [CodeAction] the response
/// is of type [CodeAction] or a Thenable that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CodeActionResolveRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: CodeAction,
}

/// A request to list project-wide symbols matching the query string given
/// by the [WorkspaceSymbolParams]. The response is
/// of type {@link SymbolInformation SymbolInformation[]} or a Thenable that
/// resolves to such.
///
/// @since 3.17.0 - support for WorkspaceSymbol in the returned data. Clients
///  need to advertise support for WorkspaceSymbols via the client capability
///  `workspace.symbol.resolveSupport`.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkspaceSymbolRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: WorkspaceSymbolParams,
}

/// A request to resolve the range inside the workspace
/// symbol's location.
///
/// @since 3.17.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkspaceSymbolResolveRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: WorkspaceSymbol,
}

/// A request to provide code lens for the given text document.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CodeLensRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: CodeLensParams,
}

/// A request to resolve a command for a given code lens.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CodeLensResolveRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: CodeLens,
}

/// A request to refresh all code actions
///
/// @since 3.16.0
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CodeLensRefreshRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: Option<()>,
}

/// A request to provide document links
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentLinkRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DocumentLinkParams,
}

/// Request to resolve additional information for a given document link. The request's
/// parameter is of type [DocumentLink] the response
/// is of type [DocumentLink] or a Thenable that resolves to such.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentLinkResolveRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DocumentLink,
}

/// A request to format a whole document.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentFormattingRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DocumentFormattingParams,
}

/// A request to format a range in a document.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentRangeFormattingRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DocumentRangeFormattingParams,
}

/// A request to format ranges in a document.
///
/// @since 3.18.0
/// @proposed
#[cfg(feature = "proposed")]
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentRangesFormattingRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DocumentRangesFormattingParams,
}

/// A request to format a document on type.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentOnTypeFormattingRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: DocumentOnTypeFormattingParams,
}

/// A request to rename a symbol.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RenameRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: RenameParams,
}

/// A request to test and perform the setup necessary for a rename.
///
/// @since 3.16 - support for default behavior
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PrepareRenameRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: PrepareRenameParams,
}

/// A request send from the client to the server to execute a command. The request might return
/// a workspace edit which the client will apply to the workspace.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExecuteCommandRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: ExecuteCommandParams,
}

/// A request sent from the server to the client to modified certain resources.
#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ApplyWorkspaceEditRequest {
    /// The version of the JSON RPC protocol.
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: LSPRequestMethods,

    /// The request id.
    pub id: LSPId,

    pub params: ApplyWorkspaceEditParams,
}
