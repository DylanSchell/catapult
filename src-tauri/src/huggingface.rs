use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Well-known providers of GGUF models on HuggingFace
pub const KNOWN_GGUF_OWNERS: &[(&str, &str)] = &[
    ("unsloth", "Dominant quantizer, Unsloth Dynamic 2.0 quants"),
    ("bartowski", "High-quality imatrix quants, large catalog"),
    ("ggml-org", "Official llama.cpp org, reference quants"),
    ("lmstudio-community", "LM Studio curated models"),
    ("mradermacher", "Prolific community quantizer, i1 variants"),
    ("MaziyarPanahi", "Diverse GGUF collection"),
    ("mmnga", "Various quants including Japanese models"),
    ("QuantFactory", "Quantized models collection"),
];

/// Curated list of recommended models with metadata
pub const RECOMMENDED_MODELS: &[RecommendedModelDef] = &[
    // ── Lightweight (runs on anything) ──────────────────────
    RecommendedModelDef {
        repo_id: "unsloth/Qwen3.5-4B-GGUF",
        filename: "Qwen3.5-4B-Q4_K_M.gguf",
        name: "Qwen 3.5 4B",
        description: "Lightweight but capable. Runs on minimal hardware.",
        params_b: 4,
        family: "Qwen 3.5",
        quant: "Q4_K_M",
        context: None,
    },
    // ── Mid-range (8-16 GB VRAM sweet spot) ─────────────────
    RecommendedModelDef {
        repo_id: "unsloth/Qwen3.5-9B-GGUF",
        filename: "Qwen3.5-9B-Q4_K_M.gguf",
        name: "Qwen 3.5 9B",
        description: "Dense 9B model. Strong all-around performance.",
        params_b: 9,
        family: "Qwen 3.5",
        quant: "Q4_K_M",
        context: None,
    },
    RecommendedModelDef {
        repo_id: "unsloth/gpt-oss-20b-GGUF",
        filename: "gpt-oss-20b-Q4_K_M.gguf",
        name: "GPT-OSS 20B (MoE, 3.6B active)",
        description: "OpenAI's open-weight MoE model. Apache 2.0 license.",
        params_b: 20,
        family: "GPT-OSS",
        quant: "Q4_K_M",
        context: None,
    },
    RecommendedModelDef {
        repo_id: "unsloth/Qwen3.5-35B-A3B-GGUF",
        filename: "Qwen3.5-35B-A3B-Q4_K_M.gguf",
        name: "Qwen 3.5 35B MoE (3B active)",
        description: "Most downloaded GGUF model. MoE with 3B active params — fast and smart.",
        params_b: 35,
        family: "Qwen 3.5",
        quant: "Q4_K_M",
        context: None,
    },
    // ── Coding ──────────────────────────────────────────────
    RecommendedModelDef {
        repo_id: "unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF",
        filename: "Qwen3-Coder-30B-A3B-Instruct-Q4_K_M.gguf",
        name: "Qwen3 Coder 30B MoE (3B active)",
        description: "Coding-focused MoE model with 3B active params.",
        params_b: 30,
        family: "Qwen3 Coder",
        quant: "Q4_K_M",
        context: None,
    },
    // ── Large (16-32 GB VRAM) ───────────────────────────────
    RecommendedModelDef {
        repo_id: "unsloth/Qwen3.5-27B-GGUF",
        filename: "Qwen3.5-27B-Q4_K_M.gguf",
        name: "Qwen 3.5 27B",
        description: "Hybrid DeltaNet architecture. High capability dense model.",
        params_b: 27,
        family: "Qwen 3.5",
        quant: "Q4_K_M",
        context: None,
    },
    RecommendedModelDef {
        repo_id: "unsloth/gemma-3-27b-it-GGUF",
        filename: "gemma-3-27b-it-Q4_K_M.gguf",
        name: "Gemma 3 27B Instruct",
        description: "Google's latest 27B model. Well-rounded and capable.",
        params_b: 27,
        family: "Gemma 3",
        quant: "Q4_K_M",
        context: None,
    },
    RecommendedModelDef {
        repo_id: "unsloth/GLM-4.7-Flash-GGUF",
        filename: "GLM-4.7-Flash-Q4_K_M.gguf",
        name: "GLM 4.7 Flash (30B MoE, 3B active)",
        description: "Zhipu AI's fast MoE model. 3B active params, very efficient.",
        params_b: 30,
        family: "GLM 4.7",
        quant: "Q4_K_M",
        context: None,
    },
    RecommendedModelDef {
        repo_id: "unsloth/Nemotron-3-Nano-30B-A3B-GGUF",
        filename: "Nemotron-3-Nano-30B-A3B-Q4_K_M.gguf",
        name: "Nemotron 3 Nano (30B MoE, 3B active)",
        description: "NVIDIA's hybrid Mamba-2/Transformer MoE. Very fast inference.",
        params_b: 30,
        family: "Nemotron 3",
        quant: "Q4_K_M",
        context: None,
    },
    // ── Extra-large (48+ GB) ────────────────────────────────
    RecommendedModelDef {
        repo_id: "unsloth/Qwen3-Coder-Next-GGUF",
        filename: "Qwen3-Coder-Next-Q4_K_M.gguf",
        name: "Qwen3 Coder Next (80B MoE, 3B active)",
        description: "Largest coding MoE. Needs ~48 GB but only 3B active params.",
        params_b: 80,
        family: "Qwen3 Coder",
        quant: "Q4_K_M",
        context: None,
    },
    RecommendedModelDef {
        repo_id: "unsloth/Qwen3.5-122B-A10B-GGUF",
        filename: "Qwen3.5-122B-A10B-Q4_K_M.gguf",
        name: "Qwen 3.5 122B MoE (10B active)",
        description: "Frontier-class MoE model. Needs ~76 GB. Top benchmark scores.",
        params_b: 122,
        family: "Qwen 3.5",
        quant: "Q4_K_M",
        context: None,
    },
];

pub struct RecommendedModelDef {
    pub repo_id: &'static str,
    pub filename: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub params_b: u32,
    pub family: &'static str,
    pub quant: &'static str,
    pub context: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HfModel {
    pub repo_id: String,
    pub name: String,
    pub author: String,
    pub tags: Vec<String>,
    pub files: Vec<HfFile>,
    pub downloads: u64,
    pub likes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HfFile {
    pub filename: String,
    pub size_bytes: u64,
    pub quant: Option<String>,
    pub download_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HfApiModel {
    id: String,
    #[serde(rename = "modelId")]
    model_id: Option<String>,
    author: Option<String>,
    tags: Option<Vec<String>>,
    downloads: Option<u64>,
    likes: Option<u64>,
    siblings: Option<Vec<HfApiFile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HfApiFile {
    rfilename: String,
    size: Option<u64>,
}

pub async fn search_models(
    client: &reqwest::Client,
    query: &str,
    owner: Option<&str>,
) -> Result<Vec<HfModel>> {
    let mut url = format!(
        "https://huggingface.co/api/models?search={}&filter=gguf&limit=30&sort=downloads",
        urlencoding_simple(query)
    );
    if let Some(owner) = owner {
        url = format!(
            "https://huggingface.co/api/models?author={}&search={}&filter=gguf&limit=50&sort=downloads",
            owner,
            urlencoding_simple(query)
        );
    }

    let response = client
        .get(&url)
        .header("User-Agent", "catapult-launcher/0.1")
        .send()
        .await
        .context("Failed to search HuggingFace")?;

    if !response.status().is_success() {
        anyhow::bail!("HuggingFace API error: {}", response.status());
    }

    let models: Vec<HfApiModel> = response.json().await.context("Failed to parse HF response")?;
    Ok(models.into_iter().map(convert_model).collect())
}

/// File entry from the HuggingFace tree API
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HfTreeEntry {
    #[serde(rename = "type")]
    entry_type: String,
    oid: Option<String>,
    size: Option<u64>,
    path: String,
}

pub async fn get_repo_files(client: &reqwest::Client, repo_id: &str) -> Result<Vec<HfFile>> {
    // Use the tree API which reliably includes file sizes
    let url = format!(
        "https://huggingface.co/api/models/{}/tree/main",
        repo_id
    );
    let response = client
        .get(&url)
        .header("User-Agent", "catapult-launcher/0.1")
        .send()
        .await
        .context("Failed to fetch repo tree")?;

    if !response.status().is_success() {
        anyhow::bail!("HuggingFace API error: {}", response.status());
    }

    let entries: Vec<HfTreeEntry> = response.json().await.context("Failed to parse tree response")?;

    let files = entries
        .into_iter()
        .filter(|e| e.entry_type == "file" && e.path.ends_with(".gguf"))
        .map(|e| {
            let quant = extract_quant(&e.path);
            let download_url = format!(
                "https://huggingface.co/{}/resolve/main/{}",
                repo_id, e.path
            );
            HfFile {
                filename: e.path,
                size_bytes: e.size.unwrap_or(0),
                quant,
                download_url,
            }
        })
        .collect();

    Ok(files)
}

fn convert_model(m: HfApiModel) -> HfModel {
    let repo_id = m.id.clone();
    let author = m.author.unwrap_or_else(|| {
        repo_id.split('/').next().unwrap_or("unknown").to_string()
    });
    let name = repo_id.split('/').last().unwrap_or(&repo_id).to_string();

    let files = m
        .siblings
        .unwrap_or_default()
        .into_iter()
        .filter(|f| f.rfilename.ends_with(".gguf"))
        .map(|f| {
            let quant = extract_quant(&f.rfilename);
            let download_url = format!(
                "https://huggingface.co/{}/resolve/main/{}",
                repo_id, f.rfilename
            );
            HfFile {
                filename: f.rfilename,
                size_bytes: f.size.unwrap_or(0),
                quant,
                download_url,
            }
        })
        .collect();

    HfModel {
        repo_id,
        name,
        author,
        tags: m.tags.unwrap_or_default(),
        files,
        downloads: m.downloads.unwrap_or(0),
        likes: m.likes.unwrap_or(0),
    }
}

pub fn extract_quant(filename: &str) -> Option<String> {
    // Match patterns like Q4_K_M, Q8_0, F16, IQ2_XXS, etc.
    let re = regex::Regex::new(r"(?i)(IQ\d[_A-Z]*|Q\d[_KM0-9A-Z]+|F16|F32|BF16)").ok()?;
    re.find(filename).map(|m| m.as_str().to_uppercase())
}

fn urlencoding_simple(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => '+'.to_string(),
            c => format!("%{:02X}", c as u32),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_quant_standard_patterns() {
        assert_eq!(extract_quant("model-Q4_K_M.gguf"), Some("Q4_K_M".to_string()));
        assert_eq!(extract_quant("model-Q8_0.gguf"), Some("Q8_0".to_string()));
        assert_eq!(extract_quant("model-F16.gguf"), Some("F16".to_string()));
        assert_eq!(extract_quant("model-BF16.gguf"), Some("BF16".to_string()));
        assert_eq!(extract_quant("model-F32.gguf"), Some("F32".to_string()));
        assert_eq!(extract_quant("model-IQ2_XXS.gguf"), Some("IQ2_XXS".to_string()));
        assert_eq!(extract_quant("model-IQ4_XS.gguf"), Some("IQ4_XS".to_string()));
        assert_eq!(extract_quant("model-Q5_K_S.gguf"), Some("Q5_K_S".to_string()));
        assert_eq!(extract_quant("model-Q6_K.gguf"), Some("Q6_K".to_string()));
    }

    #[test]
    fn extract_quant_no_match() {
        assert_eq!(extract_quant("model.gguf"), None);
        assert_eq!(extract_quant("README.md"), None);
        assert_eq!(extract_quant(""), None);
    }

    #[test]
    fn extract_quant_case_insensitive() {
        assert_eq!(extract_quant("model-q4_k_m.gguf"), Some("Q4_K_M".to_string()));
        assert_eq!(extract_quant("model-f16.gguf"), Some("F16".to_string()));
    }
}
