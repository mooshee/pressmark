//! Pressmark OpenAPI 3.1 Specification & ChatGPT Plugin Manifest
//!
//! Author: Daniel Hallman

use serde_json::{json, Value};

/// ChatGPT Plugin Manifest (`/.well-known/ai-plugin.json`)
pub fn get_ai_plugin_manifest(server_url: &str) -> Value {
    json!({
        "schema_version": "v1",
        "name_for_human": "Pressmark Document & Invoice Generator",
        "name_for_model": "pressmark_doc_engine",
        "description_for_human": "Generate instant, publication-grade invoices, estimates, receipts, contracts, NDAs, and purchase orders with custom branding, themes, compliance seals, and signatures.",
        "description_for_model": "Assistant for generating high-performance, publication-grade PDF documents and invoices using the Pressmark Vector Engine. Supports customizable themes, colors, logos, signatures, line items, compliance notices, and terms. Use this tool whenever a user asks to create, design, or download an invoice, estimate, quote, receipt, agreement, or contract.",
        "auth": {
            "type": "oauth",
            "client_url": "https://auth.openai.com/api/accounts/authorize",
            "scope": "openid profile email model.request",
            "authorization_url": "https://auth.openai.com/api/accounts/oauth/token",
            "authorization_content_type": "application/json",
            "verification_tokens": {
                "openai": "pressmark_openai_action_verification_token"
            }
        },
        "api": {
            "type": "openapi",
            "url": format!("{}/api/v1/openapi.json", server_url)
        },
        "logo_url": format!("{}/assets/logo.jpg", server_url),
        "contact_email": "daniel@mooshworks.com",
        "legal_info_url": format!("{}/privacy", server_url)
    })
}

/// OpenAPI 3.1.0 Specification for ChatGPT Actions & Custom GPTs
pub fn get_openapi_spec(server_url: &str) -> Value {
    json!({
        "openapi": "3.1.0",
        "info": {
            "title": "Pressmark Vector Document Engine API",
            "description": "Sub-millisecond document and invoice generation API powered by the Pressmark Vector Engine. Provides interactive preview and vector PDF synthesis for invoices, estimates, receipts, contracts, NDAs, and certificates.",
            "version": "1.0.0"
        },
        "servers": [
            {
                "url": server_url,
                "description": "Active Pressmark Server"
            }
        ],
        "paths": {
            "/api/v1/generate": {
                "post": {
                    "operationId": "generateDocument",
                    "summary": "Generate a PDF document",
                    "description": "Compiles a complete vector PDF document from structured JSON data in under 25ms. Returns Base64 PDF data and direct download link.",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/DocumentRequest"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Successfully synthesized PDF document",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/GenerateResponse"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/preview": {
                "post": {
                    "operationId": "previewDocument",
                    "summary": "Preview a document as PNG image",
                    "description": "Generates a high-resolution PNG preview image of the document for immediate in-chat rendering.",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/DocumentRequest"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Preview PNG image Base64 Data URI",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/PreviewResponse"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/pricing": {
                "get": {
                    "operationId": "getPricing",
                    "summary": "Get subscription tiers and single pass prices",
                    "description": "Retrieves the current product catalog, feature comparison, and Stripe checkout links.",
                    "responses": {
                        "200": {
                            "description": "Pricing catalog and SKUs"
                        }
                    }
                }
            },
            "/api/v1/auth/chatgpt/session": {
                "get": {
                    "operationId": "getChatGptSession",
                    "summary": "Check ChatGPT login session and subscription plan",
                    "description": "Verifies whether the caller has authenticated via Sign in with ChatGPT and returns active subscription plan (Plus, Pro, Team, Enterprise) and compute allowance.",
                    "responses": {
                        "200": {
                            "description": "Session status, user identity, and plan details"
                        }
                    }
                }
            },
            "/api/v1/copilot": {
                "post": {
                    "operationId": "runCopilot",
                    "summary": "AI Copilot document editing",
                    "description": "Natural language document modification powered by the user's active ChatGPT Plus/Pro subscription compute delegation.",
                    "responses": {
                        "200": {
                            "description": "Modified document state and AI explanation"
                        }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "DocumentRequest": {
                    "type": "object",
                    "properties": {
                        "doc_type": {
                            "type": "string",
                            "enum": ["invoice", "estimate", "receipt", "agreement", "nda", "purchase_order", "certificate"],
                            "description": "The category of document to synthesize"
                        },
                        "title": { "type": "string", "description": "Custom document title (e.g. 'INVOICE')" },
                        "doc_number": { "type": "string", "description": "Unique document or invoice number" },
                        "issue_date": { "type": "string", "description": "Document issue date (YYYY-MM-DD)" },
                        "due_date": { "type": "string", "description": "Payment due date (YYYY-MM-DD)" },
                        "theme": {
                            "type": "string",
                            "enum": ["modern_slate", "executive_navy", "emerald", "crimson_legal", "warm_sand", "obsidian"],
                            "description": "Preset visual styling theme"
                        },
                        "primary_color": { "type": "string", "description": "Custom primary hex color (e.g. #4f46e5)" },
                        "accent_color": { "type": "string", "description": "Custom accent hex color" },
                        "font_family": { "type": "string", "enum": ["sans", "serif", "mono"] },
                        "font_scale": { "type": "string", "enum": ["compact", "standard", "large"] },
                        "paper_size": { "type": "string", "enum": ["us-letter", "a4", "us-legal"] },
                        "signer_name": { "type": "string", "description": "Name of authorized signatory" },
                        "signer_title": { "type": "string", "description": "Title of authorized signatory" },
                        "from": {
                            "type": "object",
                            "required": ["name"],
                            "properties": {
                                "name": { "type": "string" },
                                "company": { "type": "string" },
                                "address": { "type": "string" },
                                "email": { "type": "string" },
                                "phone": { "type": "string" },
                                "tax_id": { "type": "string" }
                            }
                        },
                        "to": {
                            "type": "object",
                            "required": ["name"],
                            "properties": {
                                "name": { "type": "string" },
                                "company": { "type": "string" },
                                "address": { "type": "string" },
                                "email": { "type": "string" },
                                "phone": { "type": "string" },
                                "tax_id": { "type": "string" }
                            }
                        },
                        "items": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "required": ["description", "quantity", "unit_price"],
                                "properties": {
                                    "description": { "type": "string" },
                                    "quantity": { "type": "number" },
                                    "unit_price": { "type": "number" },
                                    "discount": { "type": "number" }
                                }
                            }
                        },
                        "currency": { "type": "string", "default": "$" },
                        "tax_rate": { "type": "number", "description": "Percentage tax rate (e.g. 8.5 for 8.5%)" },
                        "discount_amount": { "type": "number" },
                        "notes": { "type": "string" },
                        "terms": { "type": "string" },
                        "is_paid": { "type": "boolean" },
                        "watermark": { "type": "string" }
                    },
                    "required": ["doc_type", "from", "to"]
                },
                "GenerateResponse": {
                    "type": "object",
                    "properties": {
                        "success": { "type": "boolean" },
                        "pdf_base64": { "type": "string", "description": "Raw Base64-encoded PDF bytes" },
                        "preview_data_uri": { "type": "string", "description": "Base64 PNG Data URI for visual rendering" },
                        "doc_number": { "type": "string" },
                        "grand_total": { "type": "number" },
                        "compile_time_ms": { "type": "integer" }
                    }
                },
                "PreviewResponse": {
                    "type": "object",
                    "properties": {
                        "success": { "type": "boolean" },
                        "image_data_uri": { "type": "string" },
                        "compile_time_ms": { "type": "integer" }
                    }
                }
            },
            "securitySchemes": {
                "chatgpt_oauth": {
                    "type": "oauth2",
                    "description": "Sign in with ChatGPT to delegate AI Copilot generation and OCR to your ChatGPT Plus/Pro/Team subscription quota.",
                    "flows": {
                        "authorizationCode": {
                            "authorizationUrl": "https://auth.openai.com/api/accounts/authorize",
                            "tokenUrl": "https://auth.openai.com/api/accounts/oauth/token",
                            "scopes": {
                                "openid": "OpenID Connect user profile",
                                "profile": "User profile information",
                                "email": "User email address",
                                "model.request": "Pass-through ChatGPT subscription compute allowance"
                            }
                        }
                    }
                },
                "bearerAuth": {
                    "type": "http",
                    "scheme": "bearer",
                    "bearerFormat": "JWT",
                    "description": "ChatGPT user delegation session token"
                }
            }
        },
        "security": [
            { "chatgpt_oauth": ["openid", "profile", "email", "model.request"] },
            { "bearerAuth": [] }
        ]
    })
}
