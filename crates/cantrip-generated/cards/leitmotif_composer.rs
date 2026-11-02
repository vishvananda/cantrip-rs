// Leitmotif Composer
// status: planned
// requires: engine: no resolving rule on an open set: Prohibit BeBlocked
// Whenever this creature deals combat damage to a player, draw a card.
// Whenever you cast an instant or sorcery spell with mana value 5 or greater, create a token that's a copy of this creature.
// {2}{U}: Creatures named Leitmotif Composer can't be blocked this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && ((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (i64::from(q.mana_value(id))) >= (5i64)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let original_1 = { let q: &dyn Query = g; Some(asker.source) }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_1 = match original_1 { Some(original) => g.create_token_copies(original, player_1, count_1)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked" => #[allow(unused_variables)] fn lasting_rule_0(q: &dyn Query, asker: &Asker, about: Target, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Creature) && q.chars(id).name == "Leitmotif Composer") });
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let about_0 = Target::None; let until_0 = Duration::EndOfTurn;
planned!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked" => g.add_lasting_rule(item, 0, about_0, until_0););
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-resolving-rule-on-an-open-set-prohibit-be-blocked" => static LASTING_RULES: &[LastingRule] = &[LastingRule { text: "Creatures named Leitmotif Composer can't be blocked this turn.", rule: LastingRuleKind::CantBeBlocked, applies: lasting_rule_0 }];);

pub static CARD: CardDef = CardDef { name: "Leitmotif Composer", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Bard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_0_event), ability_0, "Whenever this creature deals combat damage to a player, draw a card.").targets(&[]),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "Whenever you cast an instant or sorcery spell with mana value 5 or greater, create a token that's a copy of this creature.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{2}{U}: Creatures named Leitmotif Composer can't be blocked this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
