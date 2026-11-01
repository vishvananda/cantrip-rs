// Storm Cauldron
// status: planned
// requires: engine: only a mana ability triggers on tapping for mana
// requires: player rule AdditionalLandPlays
// Each player may play an additional land during each of their turns.
// Whenever a land is tapped for mana, return it to its owner's hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }; let moved_0 = g.return_to_hand(&objects_0);
g.end_sentence();
 Ok(()) }
planned_item!("player-rule-additional-land-plays" => statics: player_rule(ability_0_0_applies, PlayerRule::ExtraLandPlays(1i64)));
planned_item!("engine-only-a-mana-ability-triggers-on-tapping-for-mana" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::TappedForMana { permanent, player, produced } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), Mana::ALL.iter().filter(|&&m| produced.get(m) > 0).fold(0u32, |types, &m| types | 1 << m as u32)]; if q.chars(id).is(CardType::Land) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Storm Cauldron", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("engine: only a mana ability triggers on tapping for mana", EventMask(0) => EventMask(EventMask::TAPPED.0)), ability_1_event), ability_1, "Whenever a land is tapped for mana, return it to its owner's hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
