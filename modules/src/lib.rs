//Library crate. Tiene todas las declaraciones que deberían usarse en el main.

// Todos los módulos del nivel actual. (Esta referencia no es necesaria que
// sea pública, porque se usa solo en este archivo).
mod abc;

// Referencias para el main.rs
pub use crate::abc::def;