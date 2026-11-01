// Glava, Five-Advents Mage
// status: planned
// requires: a limit shared by the abilities of one sentence
// requires: operation PlayersDo
// requires: operation SetX
// Whenever you cast a permanent spell with {X} in its mana cost or activate an ability with {X} in its activation cost that isn't a mana ability, you may have the value of X become 5. Do this only once each turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_or_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && q.mana_cost(id).x > 0) && !q.history().done(asker.source, AbilityRef { card: q.chars(asker.source).card, index: 200 }, asker.controller) { return Some(Fired { subject, data }); } }
 None }
fn ability_0_or_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if !g.history().done(asker.source, AbilityRef { card: g.chars(asker.source).card, index: 200 }, decider_1) && cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? { g.note_done(asker.source, AbilityRef { card: g.chars(asker.source).card, index: 200 }, decider_1);
planned!("operation-players-do" => cantrip_mechanics::players::do_this(g, asker, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, PlayersDo { any_one: false, optional: true, once_each_turn: true }, &[|g: &mut dyn Game| -> Result<(), Suspend> { planned!("operation-set-x" => g.set_x(item.triggered_by(), { let q: &dyn Query = g; 5i64 }));
g.end_sentence();
 Ok(()) }], &[])?);
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_or_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AbilityActivated { source, ability, player, .. } = event { let id = *source; let def = q.activated_ability(*ability); let subject = Target::Player(*player); let serial = match cantrip_mechanics::triggers::activated_ability_on_stack(q, event) { Some(Target::Ability(serial)) => serial, _ => 0 }; let data = [id.0, serial]; if def.is_some() && Some(asker.controller) == Some(*player) && (def.is_some_and(|d| d.cost.mana.x > 0) && !(def.is_some_and(|d| d.is_mana_ability()))) && !q.history().done(asker.source, AbilityRef { card: q.chars(asker.source).card, index: 204 }, asker.controller) { return Some(Fired { subject, data }); } }
 None }
fn ability_0_or_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if !g.history().done(asker.source, AbilityRef { card: g.chars(asker.source).card, index: 204 }, decider_5) && cantrip_mechanics::effects::ask_may(g, decider_5, asker.source, Purpose::Optional)? { g.note_done(asker.source, AbilityRef { card: g.chars(asker.source).card, index: 204 }, decider_5);
planned!("operation-players-do" => cantrip_mechanics::players::do_this(g, asker, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, PlayersDo { any_one: false, optional: true, once_each_turn: true }, &[|g: &mut dyn Game| -> Result<(), Suspend> { planned!("operation-set-x" => g.set_x(item.triggered_by(), { let q: &dyn Query = g; 5i64 }));
g.end_sentence();
 Ok(()) }], &[])?);
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Glava, Five-Advents Mage", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[planned_value!("a limit shared by the abilities of one sentence", triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_or_0_event), ability_0_or_0, "Whenever you cast a permanent spell with {X} in its mana cost or activate an ability with {X} in its activation cost that isn't a mana ability, you may have the value of X become 5. Do this only once each turn.").targets(&[]) => triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_or_0_event), ability_0_or_0, "Whenever you cast a permanent spell with {X} in its mana cost or activate an ability with {X} in its activation cost that isn't a mana ability, you may have the value of X become 5. Do this only once each turn.").targets(&[]).limit_shared_with("ability_0")), planned_value!("a limit shared by the abilities of one sentence", triggered(when_asked(EventMask(EventMask::ABILITY_ACTIVATED.0), ability_0_or_1_event), ability_0_or_1, "Whenever you cast a permanent spell with {X} in its mana cost or activate an ability with {X} in its activation cost that isn't a mana ability, you may have the value of X become 5. Do this only once each turn.").targets(&[]) => triggered(when_asked(EventMask(EventMask::ABILITY_ACTIVATED.0), ability_0_or_1_event), ability_0_or_1, "Whenever you cast a permanent spell with {X} in its mana cost or activate an ability with {X} in its activation cost that isn't a mana ability, you may have the value of X become 5. Do this only once each turn.").targets(&[]).limit_shared_with("ability_0"))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
