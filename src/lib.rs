pub mod calculator;
pub mod cli;
pub mod config;
pub mod data;
pub mod error;
pub mod models;
pub mod output;
pub mod planner;

pub use calculator::{CalorieCalculator, MacroCalculator};
pub use config::{ActivityLevel, Gender, PlanConfig};
pub use data::MealDatabase;
pub use error::{MealPlannerError, Result};
pub use models::{DailyPlan, WeeklyPlan, Goal, MacroTarget, Meal, MealType, Nutrition, Recipe};
pub use output::TerminalOutput;
pub use planner::{DailyPlanner, WeeklyPlanner};
