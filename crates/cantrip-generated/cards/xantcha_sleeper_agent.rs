// Xantcha, Sleeper Agent
// status: planned
// requires: engine: no activation by other players
// requires: event change as it enters SetController
// Xantcha enters under the control of an opponent of your choice.
// Xantcha attacks each combat if able and can't attack its owner or planeswalkers its owner controls.
// {3}: Xantcha's controller loses 2 life and you draw a card. Any player may activate this ability.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); planned!("event-change-as-it-enters-set-controller" => let players = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; if let Some(chosen) = cantrip_mechanics::choose::player(g, entering.controller, entering.object, players, "GainControl", 0)? { entering.controller = chosen; });
 Ok(()) }
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { !(attacker == asker.source && (match defender { Target::Player(p) => (Some(asker.source).map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => true && q.chars(id).is(CardType::Planeswalker) && (Some(asker.source).map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>()).contains(&q.controller(id)), _ => false })) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.must_attack += 1; }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.source).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Xantcha, Sleeper Agent", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Minion]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[planned_value!("engine: no activation by other players", ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{3}: Xantcha's controller loses 2 life and you draw a card. Any player may activate this ability.").targets(&[]) => ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{3}: Xantcha's controller loses 2 life and you draw a card. Any player may activate this ability.").targets(&[]).any_player_may_activate())], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_1_apply_0, "Xantcha attacks each combat if able and can't attack its owner or planeswalkers its owner controls.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[ability_1_1_rule], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
