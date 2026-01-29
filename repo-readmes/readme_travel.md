# Luxury Italian Escapes - AI-Powered Travel Planning

A luxury travel planning application built with **Astro**, **TypeScript**, and **LightRAG with FalkorDB** for semantic accommodation search. Plan your perfect Italian getaway with AI-powered search and ML-based ranking.

## Features

- **Semantic Search**: Natural language queries like "luxury villa near nightlife with pool"
- **ML-Powered Ranking**: Multi-stage scoring considering price, location, amenities, ratings
- **Flight Integration**: Amadeus API with mock fallback for Amsterdam to Italy routes
- **Distance Calculations**: Airport distances, taxi costs, nightlife proximity
- **Complete Cost Breakdown**: Accommodation + flights + transfers + activity transport
- **SEO Optimized**: Server-side rendering, structured data, proper meta tags
- **Alternative Suggestions**: Compare similar regions based on preferences

## Tech Stack

| Component | Technology |
|-----------|------------|
| Frontend | Astro 4.x (TypeScript) |
| Database | FalkorDB Embedded (Graph + Vector) |
| Search | LightRAG (Semantic) |
| Ranking | Custom ML Pipeline |
| Flights | Amadeus API (with mock) |
| Styling | Vanilla CSS |

## Sample Use Case

Plan a trip for **6 people** to **Puglia, Italy** from **September 17-20, 2026**:

- **Budget**: €6,000
- **Requirements**: 6 bedrooms, private pool, near nightlife
- **Departure**: Amsterdam (AMS)
- **Max per night**: €1,500

Visit `/trips/puglia-september-2026` to see the complete trip plan.

## Project Structure

```
travel/
├── docs/
│   ├── PLAN.md              # Implementation plan
│   ├── specs/SPEC.md        # Technical specification
│   └── adr/                  # Architecture Decision Records
│       ├── 001-astro-framework.md
│       ├── 002-falkordb-graph-database.md
│       ├── 003-lightrag-semantic-search.md
│       ├── 004-ml-ranking-system.md
│       └── 005-api-integrations.md
├── src/
│   ├── data/
│   │   └── accommodations.ts  # Luxury villa data
│   ├── lib/
│   │   ├── falkordb.ts        # FalkorDB embedded client
│   │   ├── lightrag.ts        # LightRAG semantic search
│   │   ├── mlRanking.ts       # ML scoring pipeline
│   │   ├── flightService.ts   # Amadeus flight API
│   │   ├── distanceService.ts # Distance/taxi calculations
│   │   └── accommodationService.ts
│   ├── layouts/
│   │   └── Layout.astro       # Base layout with SEO
│   ├── pages/
│   │   ├── index.astro        # Home page
│   │   ├── search.astro       # Search results
│   │   └── trips/
│   │       └── puglia-september-2026.astro
│   └── types/
│       └── index.ts           # TypeScript definitions
├── package.json
├── astro.config.mjs
└── tsconfig.json
```

## Getting Started

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Environment Variables

```env
# Optional - falls back to mock data
AMADEUS_API_KEY=your_amadeus_key
AMADEUS_API_SECRET=your_amadeus_secret
OPENAI_API_KEY=your_openai_key  # For real embeddings

# Feature flags
USE_MOCK_FLIGHTS=true
```

## Search Algorithm

### Retrieval Pipeline
```
Query → Embedding → Vector Search → Filter → ML Rank → Diversity → Results
```

### ML Scoring Components

| Component | Weight | Description |
|-----------|--------|-------------|
| Semantic Score | 25% | Cosine similarity |
| Price Match | 15% | Budget alignment |
| Luxury Boost | 12% | Premium property |
| Capacity Match | 10% | Guest fit |
| Rating | 10% | User ratings |
| Pool Boost | 8% | Pool availability |
| Nightlife | 8% | Proximity rating |
| Distance | 5% | City proximity |
| Airport | 5% | Travel ease |

## Regions Covered

- **Puglia** - Trulli houses, olive groves, authentic Italy
- **Amalfi Coast** - Cliffside glamour, Positano nightlife
- **Tuscany** - Wine, art, rolling hills
- **Sicily** - Ancient ruins, Taormina nightlife
- **Sardinia** - Costa Smeralda, jet-set lifestyle
- **Lake Como** - Alpine elegance, celebrity villas

## API Integrations

### Flights (Amadeus)
- Real-time flight search
- Amsterdam to Italian airports
- Pricing and availability

### Distance (Custom)
- Haversine distance calculations
- Italian taxi rate estimation
- Airport transfer costs
- Nightlife area proximity

## License

MIT
