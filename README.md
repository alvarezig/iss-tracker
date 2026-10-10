# ISS Tracker

Seguimiento en tiempo real de la Estación Espacial Internacional (ISS) desde la terminal, desarrollado en Rust.

![demo](./iss_tracker_demo.gif)

Consulta la posición real de la ISS cada 2 segundos y calcula la distancia en línea recta desde la ubicación del observador (por defecto, Buenos Aires, Argentina).

---

## Características

- Consulta periódica (cada 2 segundos) a la API pública `wheretheiss.at` para el satélite `25544` (ISS).
- Visualización en vivo de latitud, longitud, altitud, velocidad y visibilidad (`daylight` / `eclipsed`).
- Cálculo de distancia tridimensional entre el observador y la ISS, considerando la altitud orbital.
- Actualización de la terminal mediante secuencias de escape ANSI.
- Bucle asíncrono no bloqueante implementado con `tokio`.

Ejemplo de salida:

```
=== ISS Tracker (En Vivo) ===
Satelite:     iss
Latitud:      -25.9814°
Longitud:     -26.9705°
Altitud:      423.80 km
Velocidad:    27573.39 km/h
Visibilidad:  daylight
-----------------------------
Distancia:    3253.86 km
```

## Arquitectura

El proyecto se organiza en tres módulos con responsabilidades delimitadas:

```
iss_tracker/
├── Cargo.toml
└── src/
    ├── main.rs    # Bucle principal, intervalo de tokio y renderizado en terminal
    ├── client.rs  # Cliente HTTP y definición de tipos de la API
    └── geo.rs     # Coordenadas del observador y cálculo de distancia
```

Flujo de ejecución:

```
┌──────────┐   cada 2s   ┌──────────────────────────────┐
│  tokio   │ ──────────► │  GET wheretheiss.at/v1/      │
│ interval │             │  satellites/25544             │
└──────────┘             └──────────────┬───────────────┘
                                        │ JSON
                                        ▼
                              ┌──────────────────┐
                              │  IssPosition     │
                              │  lat/lon/alt/vel │
                              └────────┬─────────┘
                                       │
                       ┌───────────────┴───────────────┐
                       │                               │
                       ▼                               ▼
              ┌────────────────┐              ┌────────────────┐
              │  print terminal│              │ Observer::      │
              │  + ANSI clear  │              │ distance_to()  │
              └────────────────┘              └────────────────┘
```

### Decisiones de diseño

- **`client.rs`:** aísla la capa de red y serialización. `main.rs` no depende de URLs ni del formato JSON.
- **`geo.rs`:** concentra la lógica geométrica, lo que permite probarla de forma independiente sin acceso a la red.
- **`main.rs`:** actúa únicamente como orquestador: espera el tick, obtiene la posición, calcula la distancia y presenta los resultados.

## Cálculo de distancia

La fórmula clásica de Haversine calcula la distancia sobre la superficie terrestre. Dado que la ISS orbita a aproximadamente 420 km de altitud, ese cálculo subestima la distancia real.

El procedimiento implementado es el siguiente:

1. Cálculo del ángulo central `θ` mediante la fórmula de Haversine entre la posición del observador y el punto sub-satélite.
2. Aplicación de la ley de cosenos en tres dimensiones con radios diferenciados:

```
R1 = 6371.0 (radio terrestre)
R2 = 6371.0 + altitud_iss
dist = sqrt(R1² + R2² - 2·R1·R2·cos(θ))
```

Implementación de referencia en [`src/geo.rs`](src/geo.rs).

## Requisitos

- Rust 1.80 o superior (edition 2024).
- Conexión a internet para acceder a la API.

## Instalación y uso

```bash
# 1. Clonar el repositorio
git clone https://github.com/alvarezig/iss-tracker.git
cd iss-tracker

# 2. Ejecutar
cargo run --release
```

Para finalizar la ejecución, utilizar `Ctrl + C`.

### Configuración de la ubicación del observador

En [`src/main.rs`](src/main.rs#L12):

```rust
// Lanús, Buenos Aires por defecto
let me = Observer::new(-34.728, -58.428);

// Otros ejemplos:
// Madrid: Observer::new(40.4168, -3.7038)
// Ciudad de México: Observer::new(19.4326, -99.1332)
```

Luego de modificar las coordenadas, volver a ejecutar con `cargo run`.

## Stack tecnológico

| Tecnología | Uso |
|------------|-----|
| [Rust](https://www.rust-lang.org/) | Lenguaje de implementación |
| [tokio](https://tokio.rs/) (`full`) | Runtime asíncrono e intervalo periódico |
| [reqwest](https://docs.rs/reqwest/) (`json`) | Cliente HTTP |
| [serde](https://serde.rs/) (`derive`) | Deserialización de respuestas JSON |
| [wheretheiss.at](https://wheretheiss.at/) | API pública de posición de satélites (sin autenticación) |

## Aspectos técnicos relevantes

- Uso de `#[tokio::main]` e `interval.tick().await` para polling asíncrono.
- Modelado de dominio con `struct` e `impl`, con gestión explícita de ownership.
- Deserialización directa mediante `response.json::<IssPosition>().await`.
- Renderizado en terminal con secuencias de escape ANSI.

## Roadmap

- [ ] Notificación cuando la ISS se encuentre a menos de 500 km del observador.
- [ ] Interfaz TUI con `ratatui` y representación ASCII de la posición.
- [ ] Pruebas unitarias para `geo.rs` (casos límite: antípodas, cenit).
- [ ] Parámetros `--lat` / `--lon` por línea de comandos en lugar de valores fijos.
- [ ] Soporte para otros satélites (HST, Tiangong).

## Referencia de la API

```
GET https://api.wheretheiss.at/v1/satellites/25544
```

Respuesta (extracto):

```json
{
  "name": "iss",
  "id": 25544,
  "latitude": -25.98,
  "longitude": -26.97,
  "altitude": 423.80,
  "velocity": 27573.39,
  "visibility": "daylight"
}
```

La API no requiere autenticación. Se recomienda no reducir el intervalo entre solicitudes por debajo de 1 segundo.

---

Desarrollado por Nacho Alvarez como proyecto de aprendizaje en Rust.
