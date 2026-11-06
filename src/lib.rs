//! Pressmark - High-Performance Vector Document Synthesis Engine
//!
//! Author: Daniel Hallman

#![recursion_limit = "512"]

pub mod auth;
pub mod branding;
pub mod builder;
pub mod concepts_showcase;
pub mod downloads;
pub mod feedback;
pub mod engine;
pub mod ghl;
pub mod mcp;
pub mod models;
pub mod openapi;
pub mod pricing;
pub mod rate_limit;
pub mod server;
pub mod state_laws;
pub mod storage;
pub mod stripe;
pub mod templates;

pub use auth::{ChatGptAuthManager, ChatGptPlan, ChatGptSession, ChatGptUser};
pub use branding::{BrandingMode, DocumentBrandingConfig, ReadabilityPreset};
pub use builder::{ExecutionMode, PressmarkDocumentBuilder};
pub use engine::{generate_pdf, generate_preview_png};
pub use hallman_doc_engine::{
    inject_acroform_fields, inject_fillable_signature_acroform, should_inject_signature_acroform,
    AcroFormFieldSpec,
};
pub use models::DocumentRequest;
pub use server::{create_router, AppState};
