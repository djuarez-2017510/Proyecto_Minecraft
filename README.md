# Nether Dimension Raytracer

Un raytracer en tiempo real que renderiza una escena inspirada en la dimensión del Nether de Minecraft, implementado completamente en Rust.
## Video Funcionamiento

[![Video en YouTube](https://img.youtube.com/vi/Lquuuym7H1M/0.jpg)](https://youtu.be/Lquuuym7H1M)


## Características

- **Renderizado en Tiempo Real**: Motor de raytracing optimizado
- **Escena del Nether**: Recreación de la dimensión del Nether de Minecraft con:
  - Terreno de netherrack
  - Lagos de lava animada con emisión de luz
  - Portal del Nether con efectos animados
  - Pilares de bedrock
  - Sol dinámico
- **Materiales Avanzados**:
  - Texturas procedurales (netherrack, obsidiana, portal)
  - Materiales emisivos (lava, portal)
  - Reflexiones y transparencias
  - Sistema de roughness


## Controles

### Movimiento de Cámara
- **WASD** / **Flechas**: Mover cámara
- **Q/E** / **PageUp/PageDown**: Subir/bajar
- **Mouse (arrastrar izquierdo)**: Mirar alrededor
- **Mouse (arrastrar derecho)**: Rotar escena
- **Scroll**: Zoom in/out

### Otros
- **ESC**: Salir

## Instalación

### Requisitos Previos

- Rust 1.70+ (instalado a través de [rustup](https://rustup.rs/))
- Windows, Linux o macOS

### Compilar y Ejecutar

```bash
# Clonar el repositorio
git clone https://github.com/djuarez-2017510/Proyecto_Minecraft.git
cd Proyecto_Minecraft

# Compilar y ejecutar en modo debug
cargo run

## Estructura del Proyecto

```
Proyecto_Minecraft/
├── src/
│   ├── main.rs          # Punto de entrada, loop principal y creación de escena
│   ├── raytracer.rs     # Motor de raytracing y algoritmos de trazado
│   ├── geometry.rs      # Estructuras de geometría (Vec3, Ray, etc.)
│   ├── materials.rs     # Sistema de materiales
│   ├── shapes.rs        # Primitivas (Sphere, Cube, Plane) y BVH
│   └── texture.rs       # Sistema de texturas procedurales
├── Cargo.toml           # Dependencias del proyecto
└── README.md
```

## Tecnologías Utilizadas

- **Rust**: Lenguaje principal
- **minifb**: Biblioteca para creación de ventanas y manejo de input
- **Arquitectura Custom**: Raytracer implementado desde cero sin dependencias de motores gráficos


```

### Optimizaciones

- **Checkerboard Rendering**: Renderiza píxeles alternados en frames consecutivos
- **Temporal Reuse**: Reutiliza información del frame anterior


**djuarez-2017510**

- GitHub: [@djuarez-2017510](https://github.com/djuarez-2017510)
