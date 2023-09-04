use crate::vectors;

pub type Color = vectors::Vec3<f64>;

impl Color {
    pub fn write(self, writer: &mut impl std::io::Write, saturation: f64) -> std::io::Result<()> {
        let p = self * saturation;
        writeln!(writer, "{} {} {}", p.x as i64, p.y as i64, p.z as i64)
    }
}
