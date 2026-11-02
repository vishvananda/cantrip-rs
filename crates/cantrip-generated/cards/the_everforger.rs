// The Everforger
// status: planned
// requires: a deck rule a commander sets
// Rulebreaker — A deck with this commander can have artifact creature and Equipment cards of any color identity and any basic land cards.
// Whenever you cast an artifact creature or Equipment spell, you may copy that spell. Do this only once each turn. (The copy becomes a token.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && ((q.chars(id).is(CardType::Artifact) && q.chars(id).is(CardType::Creature)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Equipment))) && !q.history().this_turn().iter().any(|r| matches!(r.event, Event::Done { ability, player, .. } if ability == AbilityRef { card: q.chars(asker.source).card, index: 200 } && player == asker.controller)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if !g.history().done(asker.source, AbilityRef { card: g.chars(asker.source).card, index: 200 }, decider_1) && cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? { g.note_done(asker.source, AbilityRef { card: g.chars(asker.source).card, index: 200 }, decider_1);
let copier_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let spell_2: Option<ObjectId> = ({ let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()) }).first().copied();
if let Some(spell) = spell_2 { g.copy_spell(spell, copier_2, false)?; }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("a-deck-rule-a-commander-sets" => deck_rules: DeckRule::CanHaveAnyCards);
planned_item!("a-deck-rule-a-commander-sets" => deck_rules: DeckRule::CanHaveAnyBasicLandCards);

pub static CARD: CardDef = CardDef { name: "The Everforger", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Construct]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_2_event), ability_2, "Whenever you cast an artifact creature or Equipment spell, you may copy that spell. Do this only once each turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
