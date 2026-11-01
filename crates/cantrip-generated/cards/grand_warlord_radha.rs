// Grand Warlord Radha
// status: ready
// Haste
// Whenever one or more creatures you control attack, add that much mana in any combination of {R} and/or {G}. Until end of turn, you don't lose this mana as steps and phases end.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1 = { let q: &dyn Query = g; (q.event_objects(item).len() as i64).clamp(0, 255) as u8 };
for player in mana_players_1 {
if mana_1 > 0 { let colors = cantrip_mechanics::choose_colors(g, player, asker.source, ColorSet(24), mana_1)?;
for color in colors { g.add_mana_until_end_of_turn(player, color.mana(), 1); }
}
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Grand Warlord Radha", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elf,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever one or more creatures you control attack, add that much mana in any combination of {R} and/or {G}. Until end of turn, you don't lose this mana as steps and phases end.").targets(&[]).once_per_batch()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
