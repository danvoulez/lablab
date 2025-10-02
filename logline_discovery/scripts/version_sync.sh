#!/bin/bash
# 🔄 LogLine Discovery Lab - Sincronizar Versões

set -e

VERSION=$(cat VERSION)
echo "🔄 Sincronizando versão $VERSION em todos os Cargo.toml..."

# Função para atualizar versão em Cargo.toml
update_cargo_version() {
    local file="$1"
    if [ -f "$file" ]; then
        sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" "$file"
        rm "$file.bak"
        echo "✅ $file"
    fi
}

# Atualizar binários
echo ""
echo "📦 Atualizando binários..."
update_cargo_version "binaries/director/Cargo.toml"
update_cargo_version "binaries/hiv_discovery_runner/Cargo.toml"
update_cargo_version "binaries/discovery_dashboard/Cargo.toml"
update_cargo_version "binaries/job_scheduler/Cargo.toml"
update_cargo_version "binaries/job_worker/Cargo.toml"
update_cargo_version "binaries/job_client/Cargo.toml"

# Atualizar crates
echo ""
echo "📚 Atualizando crates..."
update_cargo_version "crates/spans_core/Cargo.toml"
update_cargo_version "crates/folding_runtime/Cargo.toml"
update_cargo_version "crates/causal_engine/Cargo.toml"
update_cargo_version "crates/discovery_agent/Cargo.toml"
update_cargo_version "crates/manuscript_generator/Cargo.toml"
update_cargo_version "crates/digital_twin_bridge/Cargo.toml"
update_cargo_version "crates/span_ingestor/Cargo.toml"
update_cargo_version "crates/structural_similarity/Cargo.toml"
update_cargo_version "crates/common/Cargo.toml"
update_cargo_version "crates/core/Cargo.toml"
update_cargo_version "crates/time/Cargo.toml"
update_cargo_version "crates/molecule/Cargo.toml"

# Atualizar Cargo.toml principal
echo ""
echo "🎯 Atualizando workspace principal..."
update_cargo_version "Cargo.toml"

echo ""
echo "✅ Todas as versões sincronizadas para $VERSION!"
