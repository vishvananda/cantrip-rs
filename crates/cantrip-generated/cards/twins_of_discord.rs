// Twins of Discord
// status: planned
// requires: a lasting rule reading what its ability chose
// requires: engine: no Bloodthirst
// requires: engine: no Bloodthirst granted
// Whenever you attack, choose odd or even. Creatures with mana value of that quality can't block this turn. (Zero is even.)
// Each other colorless creature you control has bloodthirst 2. (If an opponent was dealt damage this turn, that creature enters with two additional +1/+1 counters on it.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AttackersDeclared { player, count } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if *count > 0 && Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let chosen_0 = cantrip_mechanics::choose::option(g, player_0, asker.source, item.ability.card, 0..2)? - 0;
g.end_sentence();
planned!("a-lasting-rule-reading-what-its-ability-chose", CreateContinuousEffect, "affected" = "{\"Objects\":{\"args\":{\"args\":[{\"args\":\"Creature\",\"kind\":\"CardType\"},{\"args\":{\"comparison\":\"SameParity\",\"property\":\"ManaValue\",\"value\":{\"kind\":\"ChosenParity\"}},\"kind\":\"Property\"}],\"kind\":\"All\"},\"kind\":\"Battlefield\"}}", "changes" = "[{\"args\":{\"Prohibit\":{\"Block\":{\"kind\":\"Any\"}}},\"kind\":\"Rule\"}]", "duration" = "\"EndOfTurn\"", "evaluation" = "\"LiveObjects\"", "source" = { let q: &dyn Query = g; Some(asker.source) });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.source) == Some(affected)) && ((*so_far).colors.is_empty() && (*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(planned_value!("engine: no Bloodthirst granted", AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) => AbilityRef::granted_static({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))); } }
planned_item!("engine-no-bloodthirst-granted" => granted_rules: CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: Some(cantrip_mechanics::bloodthirst::enters::<2>), ..CardDef::BLANK });

pub static CARD: CardDef = CardDef { name: "Twins of Discord", token_words: &[], oracle: true, cost: ManaCost { generic: 7, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(8), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKERS_DECLARED.0), ability_0_event), ability_0, "Whenever you attack, choose odd or even. Creatures with mana value of that quality can't block this turn.").targets(&[])], statics: &[static_ability(Layer::Ability, ability_1_affects, ability_1_apply_0, "Each other colorless creature you control has bloodthirst 2.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["odd", "even"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
