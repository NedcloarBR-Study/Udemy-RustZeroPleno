mod doenca;
use doenca::Doenca;

fn main() {
  let dengue = Doenca::new(
    String::from("Dengue"),
    vec![
      String::from("Febre"),
      String::from("Dor de cabeça"),
      String::from("Dor nas articulações"),
    ],
    String::from("Vírus transmitido pelo mosquito Aedes aegypti"),
    String::from("Repouso e hidratação"),
  );
  dengue.show();
}
