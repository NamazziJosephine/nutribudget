# NutriBudget

A weekly meal planner for Health Informatics students who can cite the WHO dietary
guidelines but had instant noodles for dinner last night.

---

## You already know what a balanced diet looks like.

You study it. You can rattle off macronutrient ratios in your sleep. You know
exactly how much protein a 70kg person needs per day.

And yet — somehow — it's 9pm on a Wednesday, you have lectures until 4, a shift
until 8, an assignment due tomorrow morning, and €35 to last the week. The dorm
kitchen is free until 11pm if nobody claims it first.

You do not have time to plan.

**NutriBudget** is not a nutrition lesson. You already have those. It is a decision
support tool that takes your real constraints — your budget, your equipment, your
leftovers — and turns them into a full week of meals in seconds.

One command. No subscriptions. No recipe blogs written for people with fresh thyme
and a stand mixer. Just a plan that fits your life.

---

## Features

- 7-day meal schedule (Monday to Sunday, breakfast + lunch + dinner)
- Deduplicated shopping list with real Lidl/Aldi 2024 prices
- Daily nutrition totals: calories, protein, carbs, fat
- Estimated weekly cost with budget comparison
- Dietary restriction support: vegetarian, vegan, gluten-free, lactose-free
- Equipment awareness: works with microwave only, shared dorm kitchen, or full kitchen
- Fully offline. No API. No account. No subscription.

---

## Installation

### Option A — Download a prebuilt binary

Go to the [Releases page](../../releases) or the
[GitHub Page](https://NamazziJosephine.github.io/nutribudget)
and download the binary for your system:

| File | System |
|---|---|
| `nutribudget-x86_64` | 64-bit Linux (most laptops) |
| `nutribudget-aarch64` | ARM 64-bit (Apple Silicon via Rosetta, newer Pi) |
| `nutribudget-armv7` | ARM 32-bit (older Raspberry Pi) |

Then make it executable and run it:

```bash
chmod +x nutribudget-x86_64
./nutribudget-x86_64 --budget 35 --diet vegetarian --equipment shared-dorm-kitchen --time 20
```

### Option B — Build from source

You need [Rust installed](https://rustup.rs/). Then:

```bash
git clone https://github.com/NamazziJosephine/nutribudget
cd nutribudget
cargo build --release
./target/release/nutribudget --budget 35 --diet vegetarian --equipment shared-dorm-kitchen --time 20
```

---

## Usage

```
nutribudget [OPTIONS]

Options:
  --budget <EUR>          Weekly budget in euros (required, e.g. 35)
  --diet <DIET>           none | vegetarian | vegan | gluten-free | lactose-free
                          (default: none)
  --equipment <EQUIP>     microwave-only | shared-dorm-kitchen | full-kitchen
                          (default: shared-dorm-kitchen)
  --time <MINUTES>        Max prep time per meal, 5-60 (default: 20)
  --have <INGREDIENTS>    Comma-separated list of what you already have
  --help                  Print help
  --version               Print version
```

---

## Example

```
$ nutribudget --budget 35 --diet vegetarian --equipment shared-dorm-kitchen \
              --time 20 --have "pasta, eggs, onion"

NutriBudget -- Week Plan
Budget: EUR 35.00 | Diet: vegetarian | Equipment: shared dorm kitchen
----------------------------------------------------------------------

MON  Breakfast  Oatmeal with banana                      5 min  EUR 0.33
     Lunch      Pasta with tomato sauce                 20 min  EUR 0.48
     Dinner     Egg fried rice with vegetables          20 min  EUR 1.05
                Daily total                                       EUR 1.86

TUE  Breakfast  Toast with peanut butter                 5 min  EUR 0.42
     Lunch      Chickpea and rice salad                 10 min  EUR 0.79
     Dinner     Microwave potato with baked beans       15 min  EUR 0.63
                Daily total                                       EUR 1.84
...

----------------------------------------------------------------------
Shopping List (estimated total: EUR 28.40)
----------------------------------------------------------------------
  Apples x6                                bag  EUR 1.29
  Bananas x4                               bunch  EUR 0.60
  Canned chickpeas 400g                    can  EUR 0.69
  ...

----------------------------------------------------------------------
Daily Nutrition (7-day average)
----------------------------------------------------------------------
  Calories  1362 kcal
  Protein   50g
  Carbs     202g
  Fat       32g

----------------------------------------------------------------------
Estimated weekly cost: EUR 28.40 (within budget, EUR 6.60 to spare)

----------------------------------------------------------------------
You said you already have: pasta, eggs, onion
Cross off anything you already own before you shop.
```

---

## GitHub Page

[https://NamazziJosephine.github.io/nutribudget](https://NamazziJosephine.github.io/nutribudget)

---

## License

MIT
