// Eden, Seat of the Sanctum
// status: ready
// {T}: Add {C}.
// {5}, {T}: Mill two cards. Then you may sacrifice this land. When you do, return another target permanent card from your graveyard to your hand.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
g.add_mana(player, Mana::C, mana_0_0);}
 Ok(()) }
#[allow(unused_variables)] fn reflexive_5_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((!(Some(asker.source) == Some(id)) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static REFLEXIVE_5_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_5_target_0)];
fn reflexive_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_6 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_6 = g.return_to_hand(&objects_6);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let mills_1: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_1)).collect(); let milled_1 = g.mill_together(&mills_1);
g.end_sentence();
let decider_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_2, asker.source, Purpose::Optional)? {
let objects_3 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_3 { let controller = g.controller(object); if players_3.contains(&controller) && g.can_sacrifice(controller, object) { picks_3.push((controller, object)); } }
let sacrificed_3 = g.sacrifice_each(&picks_3);
if !sacrificed_3.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
hoisted_7.extend(sacrificed_3.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Eden, Seat of the Sanctum", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[Subtype::Town]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(ability_0_mana) }, "{T}: Add {C}."),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{5}, {T}: Mill two cards. Then you may sacrifice this land. When you do, return another target permanent card from your graveyard to your hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_5, "When you do, return another target permanent card from your graveyard to your hand.").targets(REFLEXIVE_5_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
