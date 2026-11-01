// Salacinder and Soot, Rascals
// status: planned
// requires: operation Conjure
// Prowess, Trample
// When Salacinder and Soot enters and whenever you cast an Elemental spell, choose one —
// • Conjure a card named Rite of Flame into your hand.
// • Conjure a card named Take Inventory into your hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Elemental)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_0: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_0: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_0.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Rite of Flame"), count_0, ConjureTo::Hand)?)); }
g.end_sentence();
 Ok(()) }
fn ability_2_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_1: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_1: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_1.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Take Inventory"), count_1, ConjureTo::Hand)?)); }
g.end_sentence();
 Ok(()) }
static ABILITY_2_MODES: ModalDef = modes(&[mode(ability_2_mode_0, "Conjure a card named Rite of Flame into your hand.").targets(&[]),mode(ability_2_mode_1, "Conjure a card named Take Inventory into your hand.").targets(&[])]).choose(1, 1);

pub static CARD: CardDef = CardDef { name: "Salacinder and Soot, Rascals", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::PROWESS,modal_triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0 | EventMask::SPELL_CAST.0), ability_2_event), &ABILITY_2_MODES, "When Salacinder and Soot enters and whenever you cast an Elemental spell, choose one — Conjure a card named Rite of Flame into your hand; or Conjure a card named Take Inventory into your hand.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
