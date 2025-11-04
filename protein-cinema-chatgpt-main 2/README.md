# 🧬 ProteinTrace - The Theater of Scientific Discovery

<div align="center">

![Next.js](https://img.shields.io/badge/Next.js-16.0.1-black?style=for-the-badge&logo=next.js)
![React](https://img.shields.io/badge/React-19.2.0-blue?style=for-the-badge&logo=react)
![TypeScript](https://img.shields.io/badge/TypeScript-5.0-blue?style=for-the-badge&logo=typescript)
![Tailwind CSS](https://img.shields.io/badge/Tailwind-4.0-06B6D4?style=for-the-badge&logo=tailwindcss)

**A cinematic scientific experience where protein predictions become visual performances**

[Features](#features) • [Getting Started](#getting-started) • [Deployment](#deployment)

</div>

---

## 🎭 Overview

ProteinTrace transforms complex protein computational biology into an immersive, cinematic experience. More than just a web application, it's a **theater of discovery** where each simulation is a scientific performance and every result becomes a cryptographically signed manifest of evidence.

The application combines the power of computational biology with an intuitive, mobile-first interface that makes protein structure prediction accessible to both experts and students.

## ✨ Features

### 🎬 Cinematic Experience
- **Epic Landing Page**: Stunning gradient backgrounds with floating animations and particle effects
- **Smooth Transitions**: Seamless flow from exploration to consultation mode
- **Interactive 3D Visualization**: Touch-enabled protein structure viewer with gesture support
  - 🖐️ One finger drag to rotate
  - ✌️ Two finger drag to pan
  - 🤏 Pinch to zoom
- **Responsive Design**: Mobile-first architecture that works beautifully on all devices

### 🤖 LogLine Bio - Your Scientific Assistant
- AI-powered conversational interface specialized in computational biology
- Contextual suggestions for exploration
- Real-time guidance through complex scientific concepts
- Dual-audience approach: depth for experts, clarity for beginners

### 🔬 Scientific Capabilities
- **Protein Structure Prediction**: AlphaFold-inspired modeling workflow
- **Mutation Impact Simulation**: Visualize how genetic variations affect protein structure
- **Confidence Analysis**: pLDDT (predicted Local Distance Difference Test) metrics visualization
- **Session Replay**: Timeline scrubber to review every step of your investigation
- **Analysis Dashboard**: Statistical breakdown with interactive charts

### 📜 Auditable Evidence System
- **Digital Manifesto**: Every simulation generates a cryptographically signed evidence document
- **Audit Trail**: Complete record of all computational steps
- **SHA-256 Hashing**: Structural artifacts are hashed for reproducibility
- **Timestamped Sessions**: Each investigation is uniquely identified and traceable

### 🎯 Interface Components
1. **Landing Page**: Immersive introduction to the platform
2. **Floating Chat**: Initial engagement with LogLine Bio assistant
3. **Consultation Mode**: Full-screen scientific workspace
4. **Cinema Screen**: 70% of viewport dedicated to visualization
5. **Bottom Navigation**: Quick access to Simulation, Analysis, Replay, and Manifesto tabs

## 🛠️ Tech Stack

### Core Technologies
- **Framework**: Next.js 16.0.1 (App Router)
- **Runtime**: React 19.2.0 with Server Components
- **Language**: TypeScript 5
- **Styling**: Tailwind CSS 4

### Key Libraries
- **3D Visualization**: 3Dmol.js (loaded via CDN)
- **Cryptography**: Web Crypto API (native browser support)
- **State Management**: React Hooks (useState, useEffect, useMemo)

### Development Tools
- **Linting**: ESLint with Next.js configuration
- **Build System**: Next.js Turbopack
- **Package Manager**: npm

## 🚀 Getting Started

### Prerequisites
- Node.js 20.x or higher
- npm 9.x or higher

### Installation

1. **Clone the repository**
\`\`\`bash
git clone https://github.com/danvoulez/protein-cinema-chatgpt.git
cd protein-cinema-chatgpt
\`\`\`

2. **Install dependencies**
\`\`\`bash
npm install
\`\`\`

3. **Run the development server**
\`\`\`bash
npm run dev
\`\`\`

4. **Open your browser**
Navigate to [http://localhost:3000](http://localhost:3000)

The landing page will appear with the ProteinTrace logo and floating chat will activate after 2 seconds.

### Development Commands

\`\`\`bash
# Start development server
npm run dev

# Build for production
npm run build

# Start production server
npm start

# Run linter
npm run lint
\`\`\`

## 📱 Usage Guide

### 1. Explore the Landing Page
- Admire the cinematic background with animated particles
- Read about the platform's capabilities
- Wait for the LogLine Bio chat to appear

### 2. Start a Conversation
Click on one of the suggested prompts:
- 🧬 "Quero prever uma estrutura proteica nova"
- 🔍 "Simular o impacto de uma mutação CRISPR"
- 📚 "Entender como o AlphaFold funciona"
- ⚗️ "Explorar exemplos de descobertas recentes"
- 🎓 "Sou estudante, me guie pela ciência"

### 3. Enter Consultation Mode
Once you request a simulation, the interface transforms into:
- **Cinema Screen** (top): 3D protein visualization or current analysis
- **Chat Interface** (bottom): Ongoing dialogue with LogLine Bio
- **Navigation Bar**: Switch between views

### 4. Navigate Between Views
- **🧬 Simulação**: 3D interactive protein structure
- **📊 Análise**: Statistical dashboard with pLDDT distribution
- **🎥 Replay**: Timeline of computational steps
- **📜 Manifesto**: Signed evidence document

### 5. Interact with 3D Structures
- **Rotate**: One-finger drag
- **Zoom**: Pinch gesture or mouse wheel
- **Pan**: Two-finger drag
- **Reset View**: Tap the reset button

## 🌐 Deployment

### Deploy to Vercel (Recommended)

ProteinTrace is optimized for Vercel deployment:

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https://github.com/danvoulez/protein-cinema-chatgpt)

#### Manual Deployment Steps

1. **Install Vercel CLI**
\`\`\`bash
npm install -g vercel
\`\`\`

2. **Login to Vercel**
\`\`\`bash
vercel login
\`\`\`

3. **Deploy**
\`\`\`bash
vercel
\`\`\`

4. **Deploy to Production**
\`\`\`bash
vercel --prod
\`\`\`

### Configuration

No additional environment variables are required for basic functionality. The application works out-of-the-box.

### Build Settings (Vercel)
- **Framework Preset**: Next.js
- **Build Command**: \`npm run build\`
- **Output Directory**: \`.next\`
- **Install Command**: \`npm install\`
- **Node Version**: 20.x

## 🏗️ Project Structure

\`\`\`
protein-cinema-chatgpt/
├── app/                          # Next.js App Router
│   ├── frontend/
│   │   └── landing/
│   │       └── page.tsx         # Main landing page
│   ├── globals.css              # Global styles and animations
│   ├── layout.tsx               # Root layout
│   └── page.tsx                 # Root redirect
├── components/                   # React components
│   ├── AnalysisDashboard.tsx    # Statistical analysis view
│   ├── BottomMenu.tsx           # Navigation tabs
│   ├── CinemaScreen.tsx         # Main visualization area
│   ├── CinematicBackground.tsx  # Animated background canvas
│   ├── ConsultationChat.tsx     # Chat interface in consultation mode
│   ├── ConsultationMode.tsx     # Full consultation workspace
│   ├── FloatingChat.tsx         # Initial landing page chat
│   ├── ManifestoView.tsx        # Signed evidence document
│   ├── ProteinTheater.tsx       # 3D protein visualization
│   └── SessionReplay.tsx        # Timeline scrubber
├── lib/                         # Utility libraries
│   ├── manifest.ts              # Cryptographic signing utilities
│   └── types.ts                 # TypeScript type definitions
├── public/                      # Static assets
├── next.config.ts               # Next.js configuration
├── package.json                 # Dependencies
├── tailwind.config.ts           # Tailwind configuration
└── tsconfig.json                # TypeScript configuration
\`\`\`

## 🎨 Key Design Principles

### Mobile-First Approach
Every component is designed with mobile users as the primary audience, then enhanced for larger screens.

### Performance Optimized
- Server-side rendering for initial page load
- Code splitting for optimal bundle sizes
- Lazy loading of 3D visualization library
- Efficient React hooks with proper memoization

### Accessibility
- Semantic HTML structure
- ARIA labels for screen readers
- Keyboard navigation support
- High contrast color schemes

### Progressive Enhancement
- Works without JavaScript for basic content
- Graceful fallback if 3D library fails to load
- Touch and mouse event support

## 🔐 Security & Privacy

- **No External API Calls**: All computations run in the browser
- **Client-Side Cryptography**: Uses native Web Crypto API
- **No Data Collection**: Your simulations stay in your browser
- **Reproducible Results**: Hash-based verification ensures integrity

## 🤝 Contributing

Contributions are welcome! This project follows standard GitHub flow:

1. Fork the repository
2. Create a feature branch (\`git checkout -b feature/amazing-feature\`)
3. Commit your changes (\`git commit -m 'Add amazing feature'\`)
4. Push to the branch (\`git push origin feature/amazing-feature\`)
5. Open a Pull Request

### Development Guidelines
- Follow the existing code style
- Run \`npm run lint\` before committing
- Ensure \`npm run build\` succeeds
- Write descriptive commit messages

## 📝 License

This project is open source and available under the MIT License.

## 🙏 Acknowledgments

- **AlphaFold**: Inspiration for protein structure prediction concepts
- **3Dmol.js**: Excellent 3D molecular visualization library
- **Next.js Team**: For the amazing React framework
- **Vercel**: For seamless deployment platform

## 📧 Contact

For questions, suggestions, or collaboration opportunities:
- **GitHub Issues**: [Create an issue](https://github.com/danvoulez/protein-cinema-chatgpt/issues)
- **Email**: dcamarilho@gmail.com

---

<div align="center">

**Built with ❤️ for the scientific community**

*Making computational biology accessible, one simulation at a time*

</div>
