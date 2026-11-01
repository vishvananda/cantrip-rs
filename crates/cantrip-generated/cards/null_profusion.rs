// Null Profusion
// status: planned
// requires: skipping a step of the turn
// Skip your draw step.
// Whenever you play a card, draw a card.
// Your maximum hand size is two.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && true { return Some(Fired { subject, data }); } }
if let Event::ZoneChange { new, to, reason: ZoneReason::PlayLand, .. } = event { let id = *new; let player = q.controller(id); let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if to.is(ZoneKind::Battlefield) && Some(asker.controller) == Some(player) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
planned_item!("skipping-a-step-of-the-turn" => skip_steps: SkipStep { applies: |q, asker, player| player == asker.controller, step: Step::Draw });

pub static CARD: CardDef = CardDef { name: "Null Profusion", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0 | EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Whenever you play a card, draw a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::MaximumHandSize(Some(2))], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
