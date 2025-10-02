# Task Backlog - LogLine Discovery Lab

## Phase 4 · Experience Layer (In Progress)

### Dashboard Vue.js Migration
- [ ] Set up Vue.js project structure with Vite build system
- [ ] Port existing HTTP endpoints to Vue router
- [ ] Implement component architecture (Header, Sidebar, MainView, Footer)
- [ ] Add state management (Pinia/Vuex) for span data
- [ ] Create reusable UI components (SpanCard, Timeline, MetricsChart)
- [ ] Implement responsive design with mobile support
- [ ] Add PWA manifest and service worker for offline capability
- [ ] Integrate real-time updates via WebSocket/SSE
- [ ] Add dark/light theme system
- [ ] Implement accessibility features (ARIA labels, keyboard navigation)

### Live Data Integration
- [ ] Connect dashboard to /executions/{id} API endpoint
- [ ] Implement real-time span streaming for active executions
- [ ] Add causal chain visualization with interactive nodes
- [ ] Create twin bridge divergence ticker component
- [ ] Build interactive molecular viewer integration
- [ ] Add span scrubbing timeline control
- [ ] Implement export functionality (PDF, NDJSON, LaTeX)

### Publication Mode
- [ ] Create publication mode toggle in UI
- [ ] Implement serif typography and compact layouts
- [ ] Add DOI reference system and citation manager
- [ ] Create statistical method disclosure panels
- [ ] Build compliance checklist component (GLP, FAIR, reproducibility)
- [ ] Add time-synchronized logging views
- [ ] Implement audit carousel with span IDs and commit hashes

## Phase 5 · Validation & Release

### End-to-End Testing
- [ ] Create comprehensive test suite for span ingestion pipeline
- [ ] Add property tests for causal inference accuracy
- [ ] Implement regression tests for manuscript generation
- [ ] Create integration tests for dashboard API endpoints
- [ ] Add performance benchmarks for analytics engine
- [ ] Build chaos engineering tests for 24/7 operation

### Documentation & Onboarding
- [ ] Write complete API documentation with OpenAPI spec
- [ ] Create user guide with screenshots and walkthroughs
- [ ] Build developer onboarding guide with local setup instructions
- [ ] Create troubleshooting guide for common issues
- [ ] Write architecture decision records (ADRs)
- [ ] Create deployment guide for different environments

### Release Preparation
- [ ] Set up automated release pipeline with semantic versioning
- [ ] Create bootstrap scripts for new installations
- [ ] Build Docker containers for easy deployment
- [ ] Set up monitoring and alerting for production deployments
- [ ] Create release notes template
- [ ] Plan post-release support and maintenance schedule

## Infrastructure Tasks

### CI/CD Pipeline
- [ ] Set up GitHub Actions workflow for automated testing
- [ ] Add cargo fmt/clippy checks to CI pipeline
- [ ] Implement automated security scanning
- [ ] Add performance regression detection
- [ ] Set up automated deployment to staging environment
- [ ] Create rollback procedures for failed deployments

### Monitoring & Observability
- [ ] Set up structured logging with tracing crate
- [ ] Implement metrics collection (Prometheus format)
- [ ] Add distributed tracing for microservices
- [ ] Create alerting rules for critical failures
- [ ] Build operational dashboards for system health
- [ ] Implement log aggregation and analysis

## Research & Development

### HIV Case Study Expansion
- [ ] Add gp120/gp41 interaction modeling
- [ ] Implement resistance mutation analysis
- [ ] Create vaccine candidate screening pipeline
- [ ] Add clinical trial data integration points
- [ ] Build hypothesis generation for drug targets
- [ ] Implement literature mining for validation

### Advanced Analytics
- [ ] Enhance ST-GNN model with better feature extraction
- [ ] Add uncertainty quantification to causal inference
- [ ] Implement active learning for simulation guidance
- [ ] Create ensemble methods for structural similarity
- [ ] Add temporal analysis for trajectory prediction
- [ ] Implement multi-scale modeling capabilities

## Completed Tasks ✅

### Phase 0 · Baseline
- [x] Cargo workspace with shared span schema
- [x] Basic crate structure and compilation
- [x] Seed data and initial contracts

### Phase 1 · Data Plumbing
- [x] Span ingestion from Fold outputs
- [x] NDJSON ledger mirroring
- [x] CLI ingestion commands

### Phase 2 · Analytics Core
- [x] Folding analysis implementation
- [x] Causal inference engine with multi-rule evaluation
- [x] Structural similarity with ST-GNN bridge

### Phase 3 · Orchestration & Automation
- [x] hiv_discovery_runner as orchestrator service
- [x] Automated span/contract emission
- [x] Warp rollback integration
- [x] Manuscript generation templates
- [x] 24/7 watcher and scheduler setup

### Infrastructure
- [x] Basic project structure and compilation fixes
- [x] Documentation framework established
