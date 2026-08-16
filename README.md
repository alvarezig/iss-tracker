# iss-tracker-rs

Un CLI sencillo en Rust para seguir la Estación Espacial Internacional (ISS) en tiempo real desde la terminal y calcular a cuántos kilómetros está de tu ubicación exacta. (en mi caso Buenos Aires).

---

## ¿Qué hace?

- Consulta la API pública de *Where the ISS at?* cada 2 segundos.
- Usa `tokio` para no bloquear el hilo de ejecución mientras espera las respuestas HTTP.
- Aplica trigonometría esférica (fórmula de Haversine + Teorema del Coseno) considerando la altitud orbital para darte la distancia en línea recta en 3D, no solo sobre el mapa plano.
- Limpia la pantalla en cada pulso para mostrar una interfaz ordenada en consola.

---

## Estructura del proyecto

```text
iss_tracker/
├── Cargo.toml
└── src/
    ├── client.rs   # Llamadas HTTP y tipos de la API
    ├── geo.rs      # Coordenadas del observador y cálculo de distancia
    └── main.rs     # Loop principal con Tokio
```

---

## Cómo correrlo

### 1. Clonar el repositorio
```bash
git clone [https://github.com/](https://github.com/)<tu-usuario>/iss-tracker-rs.git
cd iss-tracker-rs
```

### 2. Cambiar tus coordenadas (Opcional)
En `src/main.rs`, modificá la latitud y longitud por las tuyas:
```rust
let me = Observer::new(-34.6037, -58.3816); // Buenos Aires por defecto
```

### 3. Ejecutar
```bash
cargo run --release
```

---

## Dependencias

- **`tokio`**: Runtime asíncrono y temporizadores (`interval`).
- **`reqwest`**: Cliente HTTP para pegarle a la API.
- **`serde`**: Deserialización directa del JSON de telemetría a structs de Rust.

