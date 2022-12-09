use grid::*;
use std::{fs, collections::{HashSet, HashMap}};
use core::fmt;

fn get_file_content() -> String {
    return fs::read_to_string("./input.txt")
        .expect("File './input.txt' should have been readable");
}

fn main() {
    let file_content = get_file_content();
    let lines: Vec<&str> = file_content.split("\n").collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let size = rows * cols;
    println!("Rows: {}, Cols: {}, Size: {}", rows, cols, size);
    let mut g: Grid<u32> = Grid::new(cols, rows);
    for (r, &line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            // println!("Setting [{}, {}] to {}", c, r, ch);
            g[c][r] = ch.to_string().parse::<u32>().unwrap();
        }
    }

    let mut visible_trees = size;
    for r in 1..(g.rows()-1) {
        for c in 1..(g.cols()-1) {
            let tree_size = g[c][r];
            // println!("From [{}, {}] = {}", c, r, tree_size);
            
            // Check north
            // println!("Checking northward");
            let mut vis_north: bool = true;
            for check_r in 0..r {
                // println!("Checking [{}, {}] = {}", c, check_r, g[c][check_r]);
                if tree_size <= g[c][check_r] {
                    // println!("{} is lte {}", tree_size, g[c][check_r]);
                    vis_north = false;
                    break;
                }
            }

            // println!("Checking eastward");
            let mut vis_east: bool = true;
            for check_c in c+1..g.cols() {
                // println!("Checking [{}, {}] = {}", check_c, r, g[check_c][r]);
                if tree_size <= g[check_c][r] {
                    // println!("{} is lte {}", tree_size, g[check_c][r]);
                    vis_east = false;
                    break;
                }
            }

            // println!("Checking southward");
            let mut vis_south: bool = true;
            for check_r in r+1..g.rows() {
                // println!("Checking [{}, {}] = {}", c, check_r, g[c][check_r]);
                if tree_size <= g[c][check_r] {
                    // println!("{} is lte {}", tree_size, g[c][check_r]);
                    vis_south = false;
                    break;
                }
            }

            // println!("Checking westward");
            let mut vis_west: bool = true;
            for check_c in 0..c {
                // println!("Checking [{}, {}] = {}", check_c, r, g[check_c][r]);
                if tree_size <= g[check_c][r] {
                    // println!("{} is lte {}", tree_size, g[check_c][r]);
                    vis_west = false;
                    break;
                }
            }

            // println!("N: {}; E: {}; S: {}; W: {};", vis_north, vis_east, vis_south, vis_west);
            if (vis_north == false && vis_east == false && vis_south == false && vis_west == false) {
                // println!("Tree [{}, {}] is NOT visible", c, r);
                visible_trees -= 1;
            } else {
                // println!("Tree [{}, {}] IS visible", c, r);
            }
        }
    }

    let mut highest_view_score = 0;

    for r in 1..(g.rows()-1) {
        for c in 1..(g.cols()-1) {
            let tree_size = g[c][r];
            println!("From [{}, {}] = {}", c, r, tree_size);
            
            // Check north
            println!("Checking northward");
            let mut north_score: u32 = 0;
            for check_r in (0..r).rev() {
                println!("Checking [{}, {}] = {}", c, check_r, g[c][check_r]);
                north_score += 1;
                if tree_size <= g[c][check_r] {
                    println!("{} is lte {}", tree_size, g[c][check_r]);
                    break;
                }
            }

            println!("Checking eastward");
            let mut east_score: u32 = 0;
            for check_c in c+1..g.cols() {
                println!("Checking [{}, {}] = {}", check_c, r, g[check_c][r]);
                east_score += 1;
                if tree_size <= g[check_c][r] {
                    println!("{} is lte {}", tree_size, g[check_c][r]);
                    break;
                }
            }

            println!("Checking southward");
            let mut south_score: u32 = 0;
            for check_r in r+1..g.rows() {
                println!("Checking [{}, {}] = {}", c, check_r, g[c][check_r]);
                south_score += 1;
                if tree_size <= g[c][check_r] {
                    println!("{} is lte {}", tree_size, g[c][check_r]);
                    break;
                }
            }

            println!("Checking westward");
            let mut west_score: u32 = 0;
            for check_c in (0..c).rev() {
                println!("Checking [{}, {}] = {}", check_c, r, g[check_c][r]);
                west_score += 1;
                if tree_size <= g[check_c][r] {
                    println!("{} is lte {}", tree_size, g[check_c][r]);
                    break;
                }
            }

            println!("N: {}; E: {}; S: {}; W: {};", north_score, east_score, south_score, west_score);
            let view_score = north_score * east_score * south_score * west_score;
            if (view_score > highest_view_score) {
                println!("Tree [{}, {}] is new highest view score: {}", c, r, view_score);
                highest_view_score = view_score;
            } else {
                // println!("Tree [{}, {}] IS visible", c, r);
            }
        }
        // break;
    }

    // 2106 too high
    // 2320 too high
    // 2076 too high
    println!("Grid:\n{:#?}", g);
    println!("Rows: {}, Cols: {}", rows, cols);
    println!("Visible trees: {}", visible_trees);
    println!("Highest view score: {}", highest_view_score);

}
