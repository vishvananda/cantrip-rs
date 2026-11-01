// Su-Chi Cave Guard
// status: ready
// Vigilance
// Ward {4} (Whenever this creature becomes the target of a spell or ability an opponent controls, counter it unless that player pays {4}.)
// When this creature dies, add eight {C}. Until end of turn, you don't lose this mana as steps and phases end.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1_0 = { let q: &dyn Query = g; (8i64).max(0) as u32 };
for player in mana_players_1 {
g.add_mana_until_end_of_turn(player, Mana::C, mana_1_0);}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Su-Chi Cave Guard", token_words: &[], oracle: true, cost: ManaCost { generic: 8, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Construct]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(8), toughness: Some(8), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward::<4>("Ward {4}"),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When this creature dies, add eight {C}. Until end of turn, you don't lose this mana as steps and phases end.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
