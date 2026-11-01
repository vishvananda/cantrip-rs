// Intrepid Outlander
// status: planned
// requires: engine: venture into the dungeon reaches only Undercity
// Reach
// Pack tactics — Whenever this creature attacks, if you attacked with creatures with total power 6 or greater this combat, venture into the dungeon. (Enter the first room or advance to the next room.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { (cantrip_mechanics::history::this_combat(q).iter().filter_map(|r: &Record| match r.event { Event::Attacks { attacker, .. } if Some(q.controller(attacker)) == Some(asker.controller) => Some(i64::from(q.chars(attacker).power.unwrap_or(0))), _ => None }).sum::<i64>()) >= (6i64) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-venture-into-the-dungeon-reaches-only-undercity" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.venture(player)?; });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Intrepid Outlander", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Orc,Subtype::Ranger]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Reach]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Pack tactics — Whenever this creature attacks, if you attacked with creatures with total power 6 or greater this combat, venture into the dungeon.").targets(&[]).condition(ability_1_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
