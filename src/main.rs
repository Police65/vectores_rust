use std::fs;

fn calculate_score(scores: &mut Vec<i32>) -> f64 {
    if scores.len() < 2 {
        return 0.0;
    }
    scores.sort();
    scores.pop();
    scores.remove(0);
    let total_score = scores.iter().sum::<i32>() as f64 / scores.len() as f64;
    let artistic_score = total_score * 0.4;
    let technical_score = total_score * 0.6;
    (artistic_score + technical_score) / 2.0
}

fn main() {
    let data = fs::read_to_string("Calificacion.txt").expect("Unable to read file");
    let lines = data.split("\n");
    let mut output = String::new();
    for line in lines {
        let mut parts: Vec<&str> = line.split(",").collect();
        let name = parts.remove(0).trim();
        let scores: Vec<i32> = parts.iter().map(|score| score.trim().parse::<i32>().unwrap()).collect();
        let final_score = calculate_score(&mut scores.clone());
        output.push_str(&format!("{} {:.2}\n", name, final_score));
    }
    fs::write("Puntaje_Final.txt", output).expect("no se puede escribir archivo");
    println!("Puntaje_Final.txt salvado");
}
