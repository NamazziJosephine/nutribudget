// NutriBudget entry point
// Handles CLI argument parsing and output formatting only.
// All business logic lives in lib.rs.

use clap::Parser;
use nutribudget::{
    average_daily_nutrition, build_meal_database, build_shopping_list,
    calculate_shopping_total, day_name, filter_meals, generate_week_plan, validate_inputs,
};

// CLI argument definitions using clap derive macros.
// Each field maps to a --flag the user types in the terminal.
#[derive(Parser, Debug)]
#[command(
    name = "nutribudget",
    about = "Weekly meal planner for students who know the WHO guidelines and still ate instant noodles last night.",
    long_about = None,
    version
)]
struct Args {
    // Weekly food budget in euros.
    #[arg(long, help = "Weekly budget in euros (e.g. 35)")]
    budget: f64,

    // Dietary restriction string.
    #[arg(
        long,
        default_value = "none",
        help = "Dietary restriction: none | vegetarian | vegan | gluten-free | lactose-free"
    )]
    diet: String,

    // Available kitchen equipment string.
    #[arg(
        long,
        default_value = "shared-dorm-kitchen",
        help = "Equipment: microwave-only | shared-dorm-kitchen | full-kitchen"
    )]
    equipment: String,

    // Maximum preparation time per meal in minutes.
    #[arg(long, default_value = "20", help = "Max prep time per meal in minutes (5-60)")]
    time: u32,

    // Comma-separated list of ingredients already on hand.
    #[arg(
        long,
        default_value = "",
        help = "Ingredients you already have, comma-separated (e.g. \"pasta, eggs, onion\")"
    )]
    have: String,
}

fn main() {
    let args = Args::parse();

    let constraints = match validate_inputs(
        args.budget,
        &args.diet,
        &args.equipment,
        args.time,
        &args.have,
    ) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let all_meals = build_meal_database();
    let filtered = filter_meals(&all_meals, &constraints);

    let days = match generate_week_plan(&filtered, constraints.budget_eur) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Could not generate plan: {}", e);
            std::process::exit(1);
        }
    };

    let shopping_list = build_shopping_list(&days);
    let total_cost = calculate_shopping_total(&shopping_list);
    let avg_nutrition = average_daily_nutrition(&days);

    print_header(&constraints.equipment.display_name().to_string(), &constraints.diet.display_name().to_string(), constraints.budget_eur);
    print_week_schedule(&days);
    print_shopping_list(&shopping_list, total_cost);
    print_nutrition_summary(&avg_nutrition);
    print_cost_summary(total_cost, constraints.budget_eur);

    if !constraints.existing_ingredients.is_empty() {
        print_existing_ingredients_note(&constraints.existing_ingredients);
    }
}

// Prints the top banner with the user's constraints.
fn print_header(equipment: &str, diet: &str, budget: f64) {
    println!();
    println!("NutriBudget -- Week Plan");
    println!(
        "Budget: EUR {:.2} | Diet: {} | Equipment: {}",
        budget, diet, equipment
    );
    println!("{}", "-".repeat(70));
}

// Prints the 7-day meal schedule in the format shown in the spec.
fn print_week_schedule(days: &[nutribudget::DayPlan]) {
    println!();
    for (i, day) in days.iter().enumerate() {
        let label = day_name(i);
        println!(
            "{}  Breakfast  {:<38} {:>3} min  EUR {:.2}",
            label,
            day.breakfast.name,
            day.breakfast.prep_time_minutes,
            day.breakfast.total_cost()
        );
        println!(
            "     Lunch      {:<38} {:>3} min  EUR {:.2}",
            day.lunch.name,
            day.lunch.prep_time_minutes,
            day.lunch.total_cost()
        );
        println!(
            "     Dinner     {:<38} {:>3} min  EUR {:.2}",
            day.dinner.name,
            day.dinner.prep_time_minutes,
            day.dinner.total_cost()
        );
        println!(
            "                Daily total                                       EUR {:.2}",
            day.daily_cost()
        );
        println!();
    }
}

// Prints the deduplicated shopping list with prices.
fn print_shopping_list(list: &[nutribudget::ShoppingItem], total: f64) {
    println!("{}", "-".repeat(70));
    println!("Shopping List (estimated total: EUR {:.2})", total);
    println!("{}", "-".repeat(70));
    for item in list {
        if item.price_eur > 0.0 {
            println!("  {:<40} {}  EUR {:.2}", item.name, item.unit, item.price_eur);
        }
    }
    println!();
}

// Prints the average daily nutrition summary.
fn print_nutrition_summary(nutrition: &nutribudget::Nutrition) {
    println!("{}", "-".repeat(70));
    println!("Daily Nutrition (7-day average)");
    println!("{}", "-".repeat(70));
    println!("  Calories  {} kcal", nutrition.calories);
    println!("  Protein   {}g", nutrition.protein_g);
    println!("  Carbs     {}g", nutrition.carbs_g);
    println!("  Fat       {}g", nutrition.fat_g);
    println!();
}

// Prints the final cost summary and a budget check message.
fn print_cost_summary(total: f64, budget: f64) {
    println!("{}", "-".repeat(70));
    if total <= budget {
        println!(
            "Estimated weekly cost: EUR {:.2} (within budget, EUR {:.2} to spare)",
            total,
            budget - total
        );
    } else {
        println!(
            "Estimated weekly cost: EUR {:.2} (EUR {:.2} over your EUR {:.2} budget)",
            total,
            total - budget,
            budget
        );
        println!("Try --time 15 or --equipment microwave-only to reduce costs.");
    }
    println!();
}

// Prints a note about which existing ingredients the user listed.
fn print_existing_ingredients_note(ingredients: &[String]) {
    println!("{}", "-".repeat(70));
    println!("You said you already have: {}", ingredients.join(", "));
    println!("These have been noted. The shopping list does not subtract them automatically.");
    println!("Cross off anything you already own before you shop.");
    println!();
}
