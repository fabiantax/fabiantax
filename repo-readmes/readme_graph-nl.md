# Graph-NL

**Nederlandse Overheid Graph Database voor Onderzoeksjournalistiek**

Een graph database die alle Nederlandse overheidsinstanties, semi-overheid, en hun onderlinge relaties in kaart brengt. Ontworpen voor onderzoeksjournalistiek met geavanceerde graph algorithms.

## Doel

Met deze tool kun je:
- Verbanden ontdekken tussen overheidsorganisaties, bedrijven en personen
- Financiële stromen analyseren (subsidies, aanbestedingen, contracten)
- Belangenverstrengeling en draaideur-carrières detecteren
- Netwerk patronen en anomalieën identificeren
- Marktconcentratie bij leveranciers analyseren

## Features

- **Graph Database** met RuVector (vector search + Cypher queries)
- **30+ node types**: ministeries, gemeentes, ZBO's, zorgverzekeraars, bedrijven, personen
- **15+ relatie types**: financiert, levert_aan, bestuurder_van, toezicht_op
- **Scrapers** voor publieke databronnen
- **Investigative analyses**: draaideur-detectie, circulaire stromen, HHI-concentratie
- **CLI** voor interactieve analyse
- **Export** naar GraphML, GEXF, JSON (voor Gephi, Neo4j)

## Installatie

```bash
# Clone repository
git clone https://github.com/fabiantax/graph-nl.git
cd graph-nl

# Installeer dependencies
pip install -e ".[dev,analysis]"

# Of met uv
uv pip install -e ".[dev,analysis]"
```

### RuVector Setup

```bash
# Clone en build ruvector
git clone https://github.com/ruvnet/ruvector.git
cd ruvector
cargo build --release

# Start server
./target/release/ruvector-server
```

## Quickstart

```bash
# Scrape data van alle bronnen
graph-nl scrape all

# Of specifieke bron
graph-nl scrape almanak
graph-nl scrape zorg

# Bekijk statistieken
graph-nl stats

# Voer analyses uit
graph-nl analyze

# Zoek in de graph
graph-nl search "Achmea" --type zorgverzekeraar

# Vind pad tussen twee entiteiten
graph-nl path min-vws zv-achmea

# Exporteer voor visualisatie
graph-nl export gexf
```

## Projectstructuur

```
graph-nl/
├── src/
│   ├── models/          # Pydantic models voor nodes en edges
│   │   ├── nodes.py     # 30+ node types
│   │   └── edges.py     # 15+ relatie types
│   ├── scrapers/        # Data scrapers
│   │   ├── almanak.py   # Overheidsalmanak
│   │   ├── cbs.py       # CBS StatLine
│   │   ├── tenderned.py # Aanbestedingen
│   │   └── zorg.py      # Zorgverzekeraars & instellingen
│   ├── db/              # Database layer
│   │   ├── graph.py     # GraphDB met NetworkX
│   │   └── ruvector_client.py  # RuVector integratie
│   ├── analysis/        # Investigative analyses
│   │   ├── investigative.py    # Alle analyses
│   │   └── reports.py   # Rapport generator
│   └── cli.py           # Command-line interface
├── data/
│   ├── raw/             # Gecachte ruwe data
│   └── processed/       # Verwerkte data
├── docs/
│   ├── DATA_MODEL.md    # Uitgebreid data model
│   └── DATABRONNEN.md   # Overzicht publieke bronnen
└── tests/
```

## Data Model

### Node Types

| Categorie | Types |
|-----------|-------|
| Rijksoverheid | Ministerie, Uitvoeringsorganisatie, ZBO, Adviescollege |
| Decentraal | Provincie, Gemeente, Waterschap, Samenwerkingsverband |
| Zorg | Zorgverzekeraar, Zorginstelling, Zorgkantoor |
| Semi-overheid | Woningcorporatie, Onderwijsinstelling, Netbeheerder |
| Overig | Bedrijf, Persoon |

### Relatie Types

| Type | Beschrijving |
|------|--------------|
| `ONDERDEEL_VAN` | Hiërarchische relatie |
| `FINANCIERT` | Financiële stroom (subsidie, bijdrage) |
| `LEVERT_AAN` | Contract/aanbesteding |
| `BESTUURDER_VAN` | Persoon -> Organisatie |
| `TOEZICHT_OP` | Toezichtrelatie |
| `VERZEKERT` | Zorgverzekeraar -> Zorginstelling |

## Analyses

### Draaideur-carrières
```python
findings = analysis.find_revolving_door()
# Vindt personen die wisselen tussen overheid en private sector
```

### Belangenconflicten
```python
findings = analysis.find_conflicting_positions()
# Detecteert toezichthouder + bestuurder combinaties
```

### Circulaire Financiële Stromen
```python
findings = analysis.find_circular_flows(min_amount=100000)
# Vindt A -> B -> C -> A patronen (red flag)
```

### Marktconcentratie
```python
findings = analysis.find_concentration()
# HHI-analyse per aanbestedende dienst
```

### Cypher Queries

```cypher
// Vind alle ZBO's onder VWS
MATCH (m:ministerie {id: 'min-vws'})<-[:ONDERDEEL_VAN]-(z:zbo)
RETURN z.naam

// Vind financiële stromen > 1M
MATCH (a)-[f:FINANCIERT]->(b)
WHERE f.bedrag > 1000000
RETURN a.naam, f.bedrag, b.naam
ORDER BY f.bedrag DESC

// Vind personen met >3 bestuursposities
MATCH (p:persoon)-[r:BESTUURDER_VAN]->(o)
WITH p, COUNT(r) as posities
WHERE posities > 3
RETURN p.naam, posities
```

## Databronnen

### Open Data
- **almanak.overheid.nl** - Alle overheidsorganisaties
- **openspending.nl** - Gemeentelijke begrotingen
- **tenderned.nl** - Aanbestedingen
- **rijksfinancien.nl** - Rijksbegroting
- **jaarverantwoordingzorg.nl** - Zorg jaarverslagen
- **opendata.tweedekamer.nl** - Kamerleden, stemmingen
- **open.overheid.nl** (PLOOI) - WOO documenten

### Betaald / Beperkt
- **KvK** - Bedrijfsgegevens, UBO's
- **Company.info** - Jaarrekeningen

Zie [docs/DATABRONNEN.md](docs/DATABRONNEN.md) voor compleet overzicht.

## Zorg Sector Focus

Speciale aandacht voor de zorg sector:

```python
# Analyseer Achmea netwerk
analysis.analyze_zorgverzekeraar_network("zv-achmea")

# Resultaat:
# - Alle labels (Zilveren Kruis, FBTO, etc.)
# - Zorgkantoor regio's
# - Bestuurders en hun nevenfuncties
# - Gecontracteerde zorginstellingen
```

### Zorgverzekeraars in de Graph
- **Achmea** (29% marktaandeel): Zilveren Kruis, De Friesland, FBTO
- **VGZ** (24%): VGZ, Univé, IZZ
- **CZ** (21%): CZ, Just, OHRA
- **Menzis** (11%): Menzis, Anderzorg, HEMA

## Export & Visualisatie

```bash
# Export naar Gephi
graph-nl export gexf

# Open in Gephi voor visualisatie
```

### Aanbevolen Tools
- **Gephi** - Netwerk visualisatie
- **Neo4j** - Graph database (import via GraphML)
- **pyvis** - Interactive Python visualisatie
- **Flourish** - Online visualisatie

## Development

```bash
# Run tests
pytest

# Type checking
mypy src

# Linting
ruff check src

# Format
ruff format src
```

## Roadmap

- [ ] Live scraping van alle bronnen
- [ ] Historische data (tijdseries)
- [ ] NLP voor documentanalyse
- [ ] Web interface
- [ ] Automatische alerts bij nieuwe patronen
- [ ] Integratie met Follow the Money

## Licentie

MIT

## Bijdragen

Bijdragen welkom! Vooral:
- Nieuwe databronnen
- Verbeterde scrapers
- Extra analyses
- Documentatie

## Contact

Issues en PRs via GitHub.

---

*Dit project is bedoeld voor legitieme onderzoeksjournalistiek en transparantie.*
