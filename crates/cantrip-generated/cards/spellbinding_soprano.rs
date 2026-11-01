// Spellbinding Soprano
// status: planned
// requires: engine: no Encore
// Whenever this creature attacks, instant and sorcery spells you cast this turn cost {1} less to cast.
// Encore {3}{R} ({3}{R}, Exile this card from your graveyard: For each opponent, create a token copy that attacks that opponent this turn if able. They gain haste. Sacrifice them at the beginning of the next end step. Activate only as a sorcery.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn lasting_cost_1(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, player); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if (q.chars(spell).is(CardType::Instant) || q.chars(spell).is(CardType::Sorcery)) { CostDelta::less((1i64) as u8) } else { CostDelta::NONE } }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_0_0 { g.add_player_effect(asker.controller, source, EffectKind::SpellCostChange { card: item.ability.card, index: 0, player }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-encore" => abilities: cantrip_mechanics::encore::ability(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Encore {3}{R}"));

pub static CARD: CardDef = CardDef { name: "Spellbinding Soprano", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Bard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever this creature attacks, instant and sorcery spells you cast this turn cost {1} less to cast.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, lasting_cost_changes: &[LastingCostChange { text: "These spells cost less to cast.", change: lasting_cost_1, next: None }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
