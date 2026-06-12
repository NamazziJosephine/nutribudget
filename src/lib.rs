// NutriBudget core library
// All business logic lives here. main.rs only handles CLI parsing and calls these functions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Equipment levels ordered from least to most capable.
// A meal requiring "shared-dorm-kitchen" can be made in a full kitchen too.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Equipment {
    MicrowaveOnly,
    SharedDormKitchen,
    FullKitchen,
}

impl Equipment {
    pub fn from_str(s: &str) -> Result<Equipment, String> {
        match s {
            "microwave-only" => Ok(Equipment::MicrowaveOnly),
            "shared-dorm-kitchen" => Ok(Equipment::SharedDormKitchen),
            "full-kitchen" => Ok(Equipment::FullKitchen),
            other => Err(format!(
                "Unknown equipment '{}'. Choose: microwave-only, shared-dorm-kitchen, full-kitchen",
                other
            )),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Equipment::MicrowaveOnly => "microwave only",
            Equipment::SharedDormKitchen => "shared dorm kitchen",
            Equipment::FullKitchen => "full kitchen",
        }
    }
}

// Dietary restriction tags. A meal may have multiple tags.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DietTag {
    Vegetarian,
    Vegan,
    GlutenFree,
    LactoseFree,
}

// The diet restriction the user selects.
// "None" means no restriction: any meal is allowed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Diet {
    None,
    Vegetarian,
    Vegan,
    GlutenFree,
    LactoseFree,
}

impl Diet {
    pub fn from_str(s: &str) -> Result<Diet, String> {
        match s {
            "none" => Ok(Diet::None),
            "vegetarian" => Ok(Diet::Vegetarian),
            "vegan" => Ok(Diet::Vegan),
            "gluten-free" => Ok(Diet::GlutenFree),
            "lactose-free" => Ok(Diet::LactoseFree),
            other => Err(format!(
                "Unknown diet '{}'. Choose: none, vegetarian, vegan, gluten-free, lactose-free",
                other
            )),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Diet::None => "none",
            Diet::Vegetarian => "vegetarian",
            Diet::Vegan => "vegan",
            Diet::GlutenFree => "gluten-free",
            Diet::LactoseFree => "lactose-free",
        }
    }

    // Returns the DietTag that a meal must carry to satisfy this restriction.
    // Diet::None requires no specific tag.
    pub fn required_tag(&self) -> Option<DietTag> {
        match self {
            Diet::None => None,
            Diet::Vegetarian => Some(DietTag::Vegetarian),
            Diet::Vegan => Some(DietTag::Vegan),
            Diet::GlutenFree => Some(DietTag::GlutenFree),
            Diet::LactoseFree => Some(DietTag::LactoseFree),
        }
    }
}

// One ingredient entry: a name, cost per portion in euros, and a normalised
// shopping name used to deduplicate the shopping list across meals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub portion_cost_eur: f64,
    pub shopping_item: String,
    pub shopping_unit: String,
    pub shopping_price_eur: f64,
}

// Nutrition values per meal portion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nutrition {
    pub calories: u32,
    pub protein_g: u32,
    pub carbs_g: u32,
    pub fat_g: u32,
}

// A single meal definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meal {
    pub name: String,
    pub equipment: Equipment,
    pub prep_time_minutes: u32,
    pub ingredients: Vec<Ingredient>,
    pub nutrition: Nutrition,
    pub tags: Vec<DietTag>,
    pub meal_type: MealType,
}

impl Meal {
    // Total cost of one portion of this meal.
    pub fn total_cost(&self) -> f64 {
        self.ingredients.iter().map(|i| i.portion_cost_eur).sum()
    }
}

// Whether a meal is suitable for breakfast, lunch, or dinner.
// This keeps the schedule from assigning pasta bolognese as breakfast.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Any,
}

// The full set of user constraints collected from CLI arguments.
#[derive(Debug, Clone)]
pub struct UserConstraints {
    pub budget_eur: f64,
    pub diet: Diet,
    pub equipment: Equipment,
    pub max_time_minutes: u32,
    pub existing_ingredients: Vec<String>,
}

// One scheduled day: breakfast, lunch, dinner references into the meal list.
#[derive(Debug, Clone)]
pub struct DayPlan {
    pub breakfast: Meal,
    pub lunch: Meal,
    pub dinner: Meal,
}

impl DayPlan {
    pub fn daily_cost(&self) -> f64 {
        self.breakfast.total_cost() + self.lunch.total_cost() + self.dinner.total_cost()
    }

    pub fn daily_nutrition(&self) -> Nutrition {
        Nutrition {
            calories: self.breakfast.nutrition.calories
                + self.lunch.nutrition.calories
                + self.dinner.nutrition.calories,
            protein_g: self.breakfast.nutrition.protein_g
                + self.lunch.nutrition.protein_g
                + self.dinner.nutrition.protein_g,
            carbs_g: self.breakfast.nutrition.carbs_g
                + self.lunch.nutrition.carbs_g
                + self.dinner.nutrition.carbs_g,
            fat_g: self.breakfast.nutrition.fat_g
                + self.lunch.nutrition.fat_g
                + self.dinner.nutrition.fat_g,
        }
    }
}

// One line item on the shopping list.
#[derive(Debug, Clone)]
pub struct ShoppingItem {
    pub name: String,
    pub unit: String,
    pub price_eur: f64,
}

// The complete week plan returned to main.rs for display.
#[derive(Debug)]
pub struct WeekPlan {
    pub days: Vec<DayPlan>,
    pub shopping_list: Vec<ShoppingItem>,
    pub estimated_total_cost: f64,
}

// Hardcoded meal dataset. Prices are based on German Lidl/Aldi 2024 shelf prices.
// Every meal lists only what you actually pay per portion, not the full pack cost.
pub fn build_meal_database() -> Vec<Meal> {
    vec![
        Meal {
            name: "Oatmeal with banana".to_string(),
            equipment: Equipment::MicrowaveOnly,
            prep_time_minutes: 5,
            meal_type: MealType::Breakfast,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 380, protein_g: 10, carbs_g: 68, fat_g: 7 },
            ingredients: vec![
                Ingredient { name: "Oats 80g".to_string(), portion_cost_eur: 0.18, shopping_item: "Oats 500g".to_string(), shopping_unit: "500g bag".to_string(), shopping_price_eur: 0.89 },
                Ingredient { name: "Banana".to_string(), portion_cost_eur: 0.15, shopping_item: "Bananas x4".to_string(), shopping_unit: "bunch".to_string(), shopping_price_eur: 0.60 },
                Ingredient { name: "Water".to_string(), portion_cost_eur: 0.00, shopping_item: "Water".to_string(), shopping_unit: "tap".to_string(), shopping_price_eur: 0.00 },
            ],
        },
        Meal {
            name: "Toast with peanut butter".to_string(),
            equipment: Equipment::MicrowaveOnly,
            prep_time_minutes: 5,
            meal_type: MealType::Breakfast,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 420, protein_g: 14, carbs_g: 48, fat_g: 18 },
            ingredients: vec![
                Ingredient { name: "Toast bread 2 slices".to_string(), portion_cost_eur: 0.20, shopping_item: "Toast bread 500g".to_string(), shopping_unit: "loaf".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Peanut butter 30g".to_string(), portion_cost_eur: 0.22, shopping_item: "Peanut butter 350g".to_string(), shopping_unit: "jar".to_string(), shopping_price_eur: 1.79 },
            ],
        },
        Meal {
            name: "Greek yogurt with honey".to_string(),
            equipment: Equipment::MicrowaveOnly,
            prep_time_minutes: 3,
            meal_type: MealType::Breakfast,
            tags: vec![DietTag::Vegetarian, DietTag::GlutenFree],
            nutrition: Nutrition { calories: 310, protein_g: 18, carbs_g: 36, fat_g: 8 },
            ingredients: vec![
                Ingredient { name: "Greek yogurt 200g".to_string(), portion_cost_eur: 0.55, shopping_item: "Greek yogurt 500g".to_string(), shopping_unit: "tub".to_string(), shopping_price_eur: 1.29 },
                Ingredient { name: "Honey 15g".to_string(), portion_cost_eur: 0.12, shopping_item: "Honey 250g".to_string(), shopping_unit: "jar".to_string(), shopping_price_eur: 1.49 },
            ],
        },
        Meal {
            name: "Scrambled eggs on toast".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 10,
            meal_type: MealType::Breakfast,
            tags: vec![DietTag::Vegetarian, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 390, protein_g: 22, carbs_g: 32, fat_g: 17 },
            ingredients: vec![
                Ingredient { name: "Eggs x2".to_string(), portion_cost_eur: 0.44, shopping_item: "Eggs x10".to_string(), shopping_unit: "box".to_string(), shopping_price_eur: 1.99 },
                Ingredient { name: "Toast bread 2 slices".to_string(), portion_cost_eur: 0.20, shopping_item: "Toast bread 500g".to_string(), shopping_unit: "loaf".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Oil 5ml".to_string(), portion_cost_eur: 0.02, shopping_item: "Sunflower oil 1L".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Muesli with milk".to_string(),
            equipment: Equipment::MicrowaveOnly,
            prep_time_minutes: 3,
            meal_type: MealType::Breakfast,
            tags: vec![DietTag::Vegetarian],
            nutrition: Nutrition { calories: 400, protein_g: 13, carbs_g: 66, fat_g: 9 },
            ingredients: vec![
                Ingredient { name: "Muesli 80g".to_string(), portion_cost_eur: 0.28, shopping_item: "Muesli 750g".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.99 },
                Ingredient { name: "Milk 200ml".to_string(), portion_cost_eur: 0.22, shopping_item: "Milk 1L".to_string(), shopping_unit: "carton".to_string(), shopping_price_eur: 0.99 },
            ],
        },
        Meal {
            name: "Rye bread with cream cheese".to_string(),
            equipment: Equipment::MicrowaveOnly,
            prep_time_minutes: 5,
            meal_type: MealType::Breakfast,
            tags: vec![DietTag::Vegetarian],
            nutrition: Nutrition { calories: 340, protein_g: 11, carbs_g: 44, fat_g: 12 },
            ingredients: vec![
                Ingredient { name: "Rye bread 2 slices".to_string(), portion_cost_eur: 0.24, shopping_item: "Rye bread 500g".to_string(), shopping_unit: "loaf".to_string(), shopping_price_eur: 1.19 },
                Ingredient { name: "Cream cheese 40g".to_string(), portion_cost_eur: 0.28, shopping_item: "Cream cheese 200g".to_string(), shopping_unit: "tub".to_string(), shopping_price_eur: 0.99 },
            ],
        },
        Meal {
            name: "Overnight oats with apple".to_string(),
            equipment: Equipment::MicrowaveOnly,
            prep_time_minutes: 5,
            meal_type: MealType::Breakfast,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 360, protein_g: 9, carbs_g: 70, fat_g: 5 },
            ingredients: vec![
                Ingredient { name: "Oats 70g".to_string(), portion_cost_eur: 0.16, shopping_item: "Oats 500g".to_string(), shopping_unit: "500g bag".to_string(), shopping_price_eur: 0.89 },
                Ingredient { name: "Apple".to_string(), portion_cost_eur: 0.20, shopping_item: "Apples x6".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.29 },
                Ingredient { name: "Oat milk 200ml".to_string(), portion_cost_eur: 0.30, shopping_item: "Oat milk 1L".to_string(), shopping_unit: "carton".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Pasta with tomato sauce".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 20,
            meal_type: MealType::Lunch,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 520, protein_g: 16, carbs_g: 92, fat_g: 8 },
            ingredients: vec![
                Ingredient { name: "Pasta 100g dry".to_string(), portion_cost_eur: 0.18, shopping_item: "Pasta 500g".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 0.69 },
                Ingredient { name: "Tomato passata 150ml".to_string(), portion_cost_eur: 0.22, shopping_item: "Tomato passata 700ml".to_string(), shopping_unit: "carton".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Garlic clove".to_string(), portion_cost_eur: 0.05, shopping_item: "Garlic bulb".to_string(), shopping_unit: "bulb".to_string(), shopping_price_eur: 0.39 },
                Ingredient { name: "Oil 10ml".to_string(), portion_cost_eur: 0.03, shopping_item: "Sunflower oil 1L".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Lentil soup with bread".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 25,
            meal_type: MealType::Lunch,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::GlutenFree, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 480, protein_g: 22, carbs_g: 72, fat_g: 6 },
            ingredients: vec![
                Ingredient { name: "Red lentils 100g dry".to_string(), portion_cost_eur: 0.28, shopping_item: "Red lentils 500g".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.49 },
                Ingredient { name: "Onion".to_string(), portion_cost_eur: 0.12, shopping_item: "Onions 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Carrot".to_string(), portion_cost_eur: 0.10, shopping_item: "Carrots 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 0.89 },
                Ingredient { name: "Vegetable stock cube".to_string(), portion_cost_eur: 0.08, shopping_item: "Stock cubes x8".to_string(), shopping_unit: "box".to_string(), shopping_price_eur: 0.69 },
                Ingredient { name: "Rye bread 1 slice".to_string(), portion_cost_eur: 0.12, shopping_item: "Rye bread 500g".to_string(), shopping_unit: "loaf".to_string(), shopping_price_eur: 1.19 },
            ],
        },
        Meal {
            name: "Tuna rice bowl".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 20,
            meal_type: MealType::Lunch,
            tags: vec![DietTag::GlutenFree, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 490, protein_g: 32, carbs_g: 62, fat_g: 8 },
            ingredients: vec![
                Ingredient { name: "Rice 90g dry".to_string(), portion_cost_eur: 0.18, shopping_item: "Rice 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.09 },
                Ingredient { name: "Canned tuna 1/2 can".to_string(), portion_cost_eur: 0.55, shopping_item: "Canned tuna x4".to_string(), shopping_unit: "4-pack".to_string(), shopping_price_eur: 3.49 },
                Ingredient { name: "Corn 50g".to_string(), portion_cost_eur: 0.15, shopping_item: "Canned corn 400g".to_string(), shopping_unit: "can".to_string(), shopping_price_eur: 0.59 },
                Ingredient { name: "Soy sauce 10ml".to_string(), portion_cost_eur: 0.05, shopping_item: "Soy sauce 150ml".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 0.79 },
            ],
        },
        Meal {
            name: "Chickpea and rice salad".to_string(),
            equipment: Equipment::MicrowaveOnly,
            prep_time_minutes: 10,
            meal_type: MealType::Lunch,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::GlutenFree, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 440, protein_g: 18, carbs_g: 70, fat_g: 7 },
            ingredients: vec![
                Ingredient { name: "Canned chickpeas 200g".to_string(), portion_cost_eur: 0.35, shopping_item: "Canned chickpeas 400g".to_string(), shopping_unit: "can".to_string(), shopping_price_eur: 0.69 },
                Ingredient { name: "Cooked rice 150g".to_string(), portion_cost_eur: 0.20, shopping_item: "Rice 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.09 },
                Ingredient { name: "Cucumber 1/4".to_string(), portion_cost_eur: 0.18, shopping_item: "Cucumber".to_string(), shopping_unit: "each".to_string(), shopping_price_eur: 0.69 },
                Ingredient { name: "Olive oil 10ml".to_string(), portion_cost_eur: 0.06, shopping_item: "Olive oil 500ml".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 2.99 },
            ],
        },
        Meal {
            name: "Pasta with canned tuna and olive oil".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 20,
            meal_type: MealType::Lunch,
            tags: vec![DietTag::LactoseFree],
            nutrition: Nutrition { calories: 540, protein_g: 30, carbs_g: 72, fat_g: 12 },
            ingredients: vec![
                Ingredient { name: "Pasta 100g dry".to_string(), portion_cost_eur: 0.18, shopping_item: "Pasta 500g".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 0.69 },
                Ingredient { name: "Canned tuna 1/2 can".to_string(), portion_cost_eur: 0.55, shopping_item: "Canned tuna x4".to_string(), shopping_unit: "4-pack".to_string(), shopping_price_eur: 3.49 },
                Ingredient { name: "Olive oil 15ml".to_string(), portion_cost_eur: 0.09, shopping_item: "Olive oil 500ml".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 2.99 },
            ],
        },
        Meal {
            name: "Bean and vegetable wrap".to_string(),
            equipment: Equipment::MicrowaveOnly,
            prep_time_minutes: 10,
            meal_type: MealType::Lunch,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 460, protein_g: 18, carbs_g: 74, fat_g: 8 },
            ingredients: vec![
                Ingredient { name: "Wheat tortilla x1".to_string(), portion_cost_eur: 0.22, shopping_item: "Tortillas x8".to_string(), shopping_unit: "pack".to_string(), shopping_price_eur: 1.49 },
                Ingredient { name: "Canned kidney beans 100g".to_string(), portion_cost_eur: 0.22, shopping_item: "Canned kidney beans 400g".to_string(), shopping_unit: "can".to_string(), shopping_price_eur: 0.79 },
                Ingredient { name: "Bell pepper 1/2".to_string(), portion_cost_eur: 0.30, shopping_item: "Bell peppers x3".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.29 },
                Ingredient { name: "Tomato paste 15g".to_string(), portion_cost_eur: 0.08, shopping_item: "Tomato paste 70g".to_string(), shopping_unit: "tube".to_string(), shopping_price_eur: 0.49 },
            ],
        },
        Meal {
            name: "Potato soup".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 30,
            meal_type: MealType::Lunch,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::GlutenFree, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 410, protein_g: 10, carbs_g: 74, fat_g: 6 },
            ingredients: vec![
                Ingredient { name: "Potatoes 300g".to_string(), portion_cost_eur: 0.24, shopping_item: "Potatoes 1.5kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.49 },
                Ingredient { name: "Onion".to_string(), portion_cost_eur: 0.12, shopping_item: "Onions 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Vegetable stock cube".to_string(), portion_cost_eur: 0.08, shopping_item: "Stock cubes x8".to_string(), shopping_unit: "box".to_string(), shopping_price_eur: 0.69 },
                Ingredient { name: "Oil 10ml".to_string(), portion_cost_eur: 0.03, shopping_item: "Sunflower oil 1L".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Instant ramen upgraded with egg".to_string(),
            equipment: Equipment::MicrowaveOnly,
            prep_time_minutes: 10,
            meal_type: MealType::Lunch,
            tags: vec![DietTag::LactoseFree],
            nutrition: Nutrition { calories: 490, protein_g: 19, carbs_g: 66, fat_g: 16 },
            ingredients: vec![
                Ingredient { name: "Instant noodle pack".to_string(), portion_cost_eur: 0.29, shopping_item: "Instant noodles x5".to_string(), shopping_unit: "pack".to_string(), shopping_price_eur: 1.49 },
                Ingredient { name: "Egg x1".to_string(), portion_cost_eur: 0.22, shopping_item: "Eggs x10".to_string(), shopping_unit: "box".to_string(), shopping_price_eur: 1.99 },
                Ingredient { name: "Frozen peas 80g".to_string(), portion_cost_eur: 0.18, shopping_item: "Frozen peas 750g".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Egg fried rice with vegetables".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 20,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::Vegetarian, DietTag::LactoseFree, DietTag::GlutenFree],
            nutrition: Nutrition { calories: 560, protein_g: 20, carbs_g: 78, fat_g: 16 },
            ingredients: vec![
                Ingredient { name: "Cooked rice 200g".to_string(), portion_cost_eur: 0.22, shopping_item: "Rice 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.09 },
                Ingredient { name: "Eggs x2".to_string(), portion_cost_eur: 0.44, shopping_item: "Eggs x10".to_string(), shopping_unit: "box".to_string(), shopping_price_eur: 1.99 },
                Ingredient { name: "Frozen mixed veg 150g".to_string(), portion_cost_eur: 0.28, shopping_item: "Frozen mixed veg 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.89 },
                Ingredient { name: "Soy sauce 15ml".to_string(), portion_cost_eur: 0.08, shopping_item: "Soy sauce 150ml".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 0.79 },
                Ingredient { name: "Oil 10ml".to_string(), portion_cost_eur: 0.03, shopping_item: "Sunflower oil 1L".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Pasta bolognese with beef mince".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 30,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::LactoseFree],
            nutrition: Nutrition { calories: 640, protein_g: 38, carbs_g: 70, fat_g: 18 },
            ingredients: vec![
                Ingredient { name: "Pasta 100g dry".to_string(), portion_cost_eur: 0.18, shopping_item: "Pasta 500g".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 0.69 },
                Ingredient { name: "Beef mince 150g".to_string(), portion_cost_eur: 0.95, shopping_item: "Beef mince 500g".to_string(), shopping_unit: "pack".to_string(), shopping_price_eur: 3.29 },
                Ingredient { name: "Tomato passata 150ml".to_string(), portion_cost_eur: 0.22, shopping_item: "Tomato passata 700ml".to_string(), shopping_unit: "carton".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Onion".to_string(), portion_cost_eur: 0.12, shopping_item: "Onions 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Oil 10ml".to_string(), portion_cost_eur: 0.03, shopping_item: "Sunflower oil 1L".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Vegetarian chilli with rice".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 30,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::GlutenFree, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 560, protein_g: 22, carbs_g: 88, fat_g: 7 },
            ingredients: vec![
                Ingredient { name: "Canned kidney beans 200g".to_string(), portion_cost_eur: 0.35, shopping_item: "Canned kidney beans 400g".to_string(), shopping_unit: "can".to_string(), shopping_price_eur: 0.79 },
                Ingredient { name: "Canned chopped tomatoes".to_string(), portion_cost_eur: 0.45, shopping_item: "Canned chopped tomatoes 400g".to_string(), shopping_unit: "can".to_string(), shopping_price_eur: 0.49 },
                Ingredient { name: "Rice 90g dry".to_string(), portion_cost_eur: 0.18, shopping_item: "Rice 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.09 },
                Ingredient { name: "Onion".to_string(), portion_cost_eur: 0.12, shopping_item: "Onions 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Bell pepper 1/2".to_string(), portion_cost_eur: 0.30, shopping_item: "Bell peppers x3".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.29 },
                Ingredient { name: "Cumin and paprika pinch".to_string(), portion_cost_eur: 0.05, shopping_item: "Cumin 50g".to_string(), shopping_unit: "jar".to_string(), shopping_price_eur: 0.89 },
            ],
        },
        Meal {
            name: "Microwave potato with baked beans".to_string(),
            equipment: Equipment::MicrowaveOnly,
            prep_time_minutes: 15,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::GlutenFree, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 510, protein_g: 20, carbs_g: 88, fat_g: 4 },
            ingredients: vec![
                Ingredient { name: "Large potato".to_string(), portion_cost_eur: 0.25, shopping_item: "Potatoes 1.5kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.49 },
                Ingredient { name: "Canned baked beans 200g".to_string(), portion_cost_eur: 0.38, shopping_item: "Baked beans 400g".to_string(), shopping_unit: "can".to_string(), shopping_price_eur: 0.69 },
            ],
        },
        Meal {
            name: "Chicken breast with rice and broccoli".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 25,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::GlutenFree, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 530, protein_g: 48, carbs_g: 58, fat_g: 8 },
            ingredients: vec![
                Ingredient { name: "Chicken breast 150g".to_string(), portion_cost_eur: 0.85, shopping_item: "Chicken breast 600g".to_string(), shopping_unit: "pack".to_string(), shopping_price_eur: 3.49 },
                Ingredient { name: "Rice 90g dry".to_string(), portion_cost_eur: 0.18, shopping_item: "Rice 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.09 },
                Ingredient { name: "Frozen broccoli 150g".to_string(), portion_cost_eur: 0.25, shopping_item: "Frozen broccoli 750g".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.29 },
                Ingredient { name: "Oil 10ml".to_string(), portion_cost_eur: 0.03, shopping_item: "Sunflower oil 1L".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Veggie stir fry with noodles".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 20,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 510, protein_g: 14, carbs_g: 86, fat_g: 10 },
            ingredients: vec![
                Ingredient { name: "Egg noodles 100g dry".to_string(), portion_cost_eur: 0.28, shopping_item: "Egg noodles 250g".to_string(), shopping_unit: "pack".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Frozen mixed veg 200g".to_string(), portion_cost_eur: 0.38, shopping_item: "Frozen mixed veg 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.89 },
                Ingredient { name: "Soy sauce 20ml".to_string(), portion_cost_eur: 0.10, shopping_item: "Soy sauce 150ml".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 0.79 },
                Ingredient { name: "Oil 10ml".to_string(), portion_cost_eur: 0.03, shopping_item: "Sunflower oil 1L".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Red lentil dahl with rice".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 30,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::GlutenFree, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 540, protein_g: 26, carbs_g: 86, fat_g: 7 },
            ingredients: vec![
                Ingredient { name: "Red lentils 100g dry".to_string(), portion_cost_eur: 0.28, shopping_item: "Red lentils 500g".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.49 },
                Ingredient { name: "Rice 80g dry".to_string(), portion_cost_eur: 0.16, shopping_item: "Rice 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.09 },
                Ingredient { name: "Onion".to_string(), portion_cost_eur: 0.12, shopping_item: "Onions 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Cumin and turmeric".to_string(), portion_cost_eur: 0.06, shopping_item: "Cumin 50g".to_string(), shopping_unit: "jar".to_string(), shopping_price_eur: 0.89 },
                Ingredient { name: "Canned chopped tomatoes half can".to_string(), portion_cost_eur: 0.25, shopping_item: "Canned chopped tomatoes 400g".to_string(), shopping_unit: "can".to_string(), shopping_price_eur: 0.49 },
                Ingredient { name: "Oil 10ml".to_string(), portion_cost_eur: 0.03, shopping_item: "Sunflower oil 1L".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Omelette with cheese and onion".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 15,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::Vegetarian, DietTag::GlutenFree],
            nutrition: Nutrition { calories: 430, protein_g: 28, carbs_g: 8, fat_g: 30 },
            ingredients: vec![
                Ingredient { name: "Eggs x3".to_string(), portion_cost_eur: 0.66, shopping_item: "Eggs x10".to_string(), shopping_unit: "box".to_string(), shopping_price_eur: 1.99 },
                Ingredient { name: "Cheddar cheese 40g".to_string(), portion_cost_eur: 0.38, shopping_item: "Cheddar 400g".to_string(), shopping_unit: "block".to_string(), shopping_price_eur: 2.99 },
                Ingredient { name: "Onion half".to_string(), portion_cost_eur: 0.06, shopping_item: "Onions 1kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Oil 5ml".to_string(), portion_cost_eur: 0.02, shopping_item: "Sunflower oil 1L".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Microwave rice with chickpeas and tomato".to_string(),
            equipment: Equipment::MicrowaveOnly,
            prep_time_minutes: 15,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::GlutenFree, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 490, protein_g: 18, carbs_g: 82, fat_g: 6 },
            ingredients: vec![
                Ingredient { name: "Microwave rice pouch 200g".to_string(), portion_cost_eur: 0.55, shopping_item: "Microwave rice pouches x4".to_string(), shopping_unit: "pack".to_string(), shopping_price_eur: 1.99 },
                Ingredient { name: "Canned chickpeas 200g".to_string(), portion_cost_eur: 0.35, shopping_item: "Canned chickpeas 400g".to_string(), shopping_unit: "can".to_string(), shopping_price_eur: 0.69 },
                Ingredient { name: "Canned chopped tomatoes half can".to_string(), portion_cost_eur: 0.25, shopping_item: "Canned chopped tomatoes 400g".to_string(), shopping_unit: "can".to_string(), shopping_price_eur: 0.49 },
            ],
        },
        Meal {
            name: "Pasta with pesto and parmesan".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 15,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::Vegetarian],
            nutrition: Nutrition { calories: 580, protein_g: 18, carbs_g: 72, fat_g: 22 },
            ingredients: vec![
                Ingredient { name: "Pasta 100g dry".to_string(), portion_cost_eur: 0.18, shopping_item: "Pasta 500g".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 0.69 },
                Ingredient { name: "Pesto 30g".to_string(), portion_cost_eur: 0.35, shopping_item: "Pesto 190g".to_string(), shopping_unit: "jar".to_string(), shopping_price_eur: 1.99 },
                Ingredient { name: "Parmesan 15g".to_string(), portion_cost_eur: 0.28, shopping_item: "Parmesan 100g".to_string(), shopping_unit: "pack".to_string(), shopping_price_eur: 1.79 },
            ],
        },
        Meal {
            name: "Black bean tacos".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 20,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::Vegetarian, DietTag::Vegan, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 500, protein_g: 20, carbs_g: 78, fat_g: 10 },
            ingredients: vec![
                Ingredient { name: "Wheat tortillas x2".to_string(), portion_cost_eur: 0.44, shopping_item: "Tortillas x8".to_string(), shopping_unit: "pack".to_string(), shopping_price_eur: 1.49 },
                Ingredient { name: "Canned black beans 200g".to_string(), portion_cost_eur: 0.38, shopping_item: "Canned black beans 400g".to_string(), shopping_unit: "can".to_string(), shopping_price_eur: 0.79 },
                Ingredient { name: "Bell pepper half".to_string(), portion_cost_eur: 0.30, shopping_item: "Bell peppers x3".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.29 },
                Ingredient { name: "Cumin and paprika".to_string(), portion_cost_eur: 0.05, shopping_item: "Cumin 50g".to_string(), shopping_unit: "jar".to_string(), shopping_price_eur: 0.89 },
                Ingredient { name: "Oil 10ml".to_string(), portion_cost_eur: 0.03, shopping_item: "Sunflower oil 1L".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Sardines with mashed potato".to_string(),
            equipment: Equipment::SharedDormKitchen,
            prep_time_minutes: 20,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::GlutenFree, DietTag::LactoseFree],
            nutrition: Nutrition { calories: 520, protein_g: 34, carbs_g: 52, fat_g: 18 },
            ingredients: vec![
                Ingredient { name: "Canned sardines in oil".to_string(), portion_cost_eur: 0.65, shopping_item: "Canned sardines".to_string(), shopping_unit: "can".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Potatoes 250g".to_string(), portion_cost_eur: 0.20, shopping_item: "Potatoes 1.5kg".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 1.49 },
                Ingredient { name: "Milk 50ml".to_string(), portion_cost_eur: 0.05, shopping_item: "Milk 1L".to_string(), shopping_unit: "carton".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Oil 5ml".to_string(), portion_cost_eur: 0.02, shopping_item: "Sunflower oil 1L".to_string(), shopping_unit: "bottle".to_string(), shopping_price_eur: 1.29 },
            ],
        },
        Meal {
            name: "Microwave mac and cheese".to_string(),
            equipment: Equipment::MicrowaveOnly,
            prep_time_minutes: 10,
            meal_type: MealType::Dinner,
            tags: vec![DietTag::Vegetarian],
            nutrition: Nutrition { calories: 550, protein_g: 20, carbs_g: 74, fat_g: 18 },
            ingredients: vec![
                Ingredient { name: "Pasta 100g dry".to_string(), portion_cost_eur: 0.18, shopping_item: "Pasta 500g".to_string(), shopping_unit: "bag".to_string(), shopping_price_eur: 0.69 },
                Ingredient { name: "Cheddar cheese 60g".to_string(), portion_cost_eur: 0.58, shopping_item: "Cheddar 400g".to_string(), shopping_unit: "block".to_string(), shopping_price_eur: 2.99 },
                Ingredient { name: "Milk 100ml".to_string(), portion_cost_eur: 0.10, shopping_item: "Milk 1L".to_string(), shopping_unit: "carton".to_string(), shopping_price_eur: 0.99 },
                Ingredient { name: "Butter 10g".to_string(), portion_cost_eur: 0.08, shopping_item: "Butter 250g".to_string(), shopping_unit: "block".to_string(), shopping_price_eur: 1.79 },
            ],
        },
    ]
}

// Filters the meal database to only meals that satisfy all constraints.
// Equipment check: user equipment level must be >= meal requirement.
// Diet check: if user selected a diet, meal must carry the matching tag.
// Time check: meal prep time must be <= user max time.
pub fn filter_meals(meals: &[Meal], constraints: &UserConstraints) -> Vec<Meal> {
    meals
        .iter()
        .filter(|meal| {
            meal.equipment <= constraints.equipment
                && meal.prep_time_minutes <= constraints.max_time_minutes
                && diet_satisfied(meal, &constraints.diet)
        })
        .cloned()
        .collect()
}

// Returns true if the meal satisfies the dietary requirement.
fn diet_satisfied(meal: &Meal, diet: &Diet) -> bool {
    match diet.required_tag() {
        None => true,
        Some(required) => meal.tags.contains(&required),
    }
}

// Selects one meal of a given type from the filtered list using round-robin day index.
pub fn pick_meal(meals: &[Meal], meal_type: &MealType, day_index: usize) -> Option<Meal> {
    let candidates: Vec<&Meal> = meals
        .iter()
        .filter(|m| &m.meal_type == meal_type || m.meal_type == MealType::Any)
        .collect();

    if candidates.is_empty() {
        return None;
    }

    let index = day_index % candidates.len();
    Some(candidates[index].clone())
}

// Generates a 7-day meal plan from the filtered meal list.
// Returns an error string if no meals pass the filter for any meal type.
pub fn generate_week_plan(filtered_meals: &[Meal], budget: f64) -> Result<Vec<DayPlan>, String> {
    let breakfasts: Vec<Meal> = filtered_meals
        .iter()
        .filter(|m| m.meal_type == MealType::Breakfast || m.meal_type == MealType::Any)
        .cloned()
        .collect();

    let lunches: Vec<Meal> = filtered_meals
        .iter()
        .filter(|m| m.meal_type == MealType::Lunch || m.meal_type == MealType::Any)
        .cloned()
        .collect();

    let dinners: Vec<Meal> = filtered_meals
        .iter()
        .filter(|m| m.meal_type == MealType::Dinner || m.meal_type == MealType::Any)
        .cloned()
        .collect();

    if breakfasts.is_empty() {
        return Err("No breakfasts match your constraints. Try relaxing equipment or diet.".to_string());
    }
    if lunches.is_empty() {
        return Err("No lunches match your constraints. Try relaxing equipment or diet.".to_string());
    }
    if dinners.is_empty() {
        return Err("No dinners match your constraints. Try relaxing equipment or diet.".to_string());
    }

    let mut days = Vec::with_capacity(7);
    let mut total_cost = 0.0;

    for day_index in 0..7 {
        let breakfast = pick_meal(&breakfasts, &MealType::Breakfast, day_index)
            .ok_or("Could not pick breakfast")?;
        let lunch = pick_meal(&lunches, &MealType::Lunch, day_index)
            .ok_or("Could not pick lunch")?;
        let dinner = pick_meal(&dinners, &MealType::Dinner, day_index)
            .ok_or("Could not pick dinner")?;

        let day = DayPlan { breakfast, lunch, dinner };
        total_cost += day.daily_cost();
        days.push(day);
    }

    let _ = (budget, total_cost);
    Ok(days)
}

// Builds a deduplicated shopping list from all meals in the plan.
// Shopping items with the same name are included only once.
pub fn build_shopping_list(days: &[DayPlan]) -> Vec<ShoppingItem> {
    let mut seen: HashMap<String, ShoppingItem> = HashMap::new();

    for day in days {
        for meal in [&day.breakfast, &day.lunch, &day.dinner] {
            for ingredient in &meal.ingredients {
                if ingredient.shopping_price_eur > 0.0 {
                    seen.entry(ingredient.shopping_item.clone()).or_insert(ShoppingItem {
                        name: ingredient.shopping_item.clone(),
                        unit: ingredient.shopping_unit.clone(),
                        price_eur: ingredient.shopping_price_eur,
                    });
                }
            }
        }
    }

    let mut list: Vec<ShoppingItem> = seen.into_values().collect();
    list.sort_by(|a, b| a.name.cmp(&b.name));
    list
}

// Sums up the total estimated cost from the shopping list.
pub fn calculate_shopping_total(list: &[ShoppingItem]) -> f64 {
    list.iter().map(|item| item.price_eur).sum()
}

// Calculates average daily nutrition across the entire week plan.
pub fn average_daily_nutrition(days: &[DayPlan]) -> Nutrition {
    if days.is_empty() {
        return Nutrition { calories: 0, protein_g: 0, carbs_g: 0, fat_g: 0 };
    }

    let total_calories: u32 = days.iter().map(|d| d.daily_nutrition().calories).sum();
    let total_protein: u32 = days.iter().map(|d| d.daily_nutrition().protein_g).sum();
    let total_carbs: u32 = days.iter().map(|d| d.daily_nutrition().carbs_g).sum();
    let total_fat: u32 = days.iter().map(|d| d.daily_nutrition().fat_g).sum();

    let n = days.len() as u32;
    Nutrition {
        calories: total_calories / n,
        protein_g: total_protein / n,
        carbs_g: total_carbs / n,
        fat_g: total_fat / n,
    }
}

// Validates all user inputs and returns a populated UserConstraints or a clear error message.
pub fn validate_inputs(
    budget: f64,
    diet_str: &str,
    equipment_str: &str,
    time_minutes: u32,
    have_str: &str,
) -> Result<UserConstraints, String> {
    if budget < 10.0 || budget > 200.0 {
        return Err(format!(
            "Budget must be between 10 and 200 euros. You entered {:.2}.",
            budget
        ));
    }

    let diet = Diet::from_str(diet_str)?;
    let equipment = Equipment::from_str(equipment_str)?;

    if time_minutes < 5 || time_minutes > 60 {
        return Err(format!(
            "Max time must be between 5 and 60 minutes. You entered {} minutes.",
            time_minutes
        ));
    }

    let existing_ingredients: Vec<String> = if have_str.trim().is_empty() {
        vec![]
    } else {
        have_str
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect()
    };

    Ok(UserConstraints {
        budget_eur: budget,
        diet,
        equipment,
        max_time_minutes: time_minutes,
        existing_ingredients,
    })
}

// Returns the display name for a day of the week by zero-based index.
pub fn day_name(index: usize) -> &'static str {
    match index {
        0 => "MON",
        1 => "TUE",
        2 => "WED",
        3 => "THU",
        4 => "FRI",
        5 => "SAT",
        6 => "SUN",
        _ => "???",
    }
}
