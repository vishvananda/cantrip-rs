// Locus of Enlightenment // The Enigma Jewel
// status: planned
// requires: object filter HasActivatedAbility
// requires: static change GainAbilitiesOf
// Locus of Enlightenment has each activated ability of the exiled cards used to craft it. You may activate each of those abilities only once each turn.
// Whenever you activate an ability that isn't a mana ability, copy it. You may choose new targets for the copy.
// The Enigma Jewel enters tapped.
// {T}: Add {C}{C}. Spend this mana only to activate abilities.
// Craft with four or more nonlands with activated abilities {8}{U} ({8}{U}, Exile this artifact, Exile the four or more from among other permanents you control and/or cards in your graveyard: Return this card transformed under its owner's control. Craft only as a sorcery.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
#[allow(unused_variables)] fn ability_2_1(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!q.chars(id).is(CardType::Land)) && (planned_value!("object filter HasActivatedAbility", false => q.card_of(id).activated.iter().any(|d| matches(d)))) }
static ABILITY_2_MATERIALS: Materials = cantrip_mechanics::craft::materials(4, true, ability_2_1, "nonland with an activated ability");

pub static CARD: CardDef = CardDef { name: "The Enigma Jewel", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 2]), ..add_each(&[]) }.only_for(ManaRestriction::ABILITIES), "{T}: Add {C}{C}. Spend this mana only to activate abilities."),cantrip_mechanics::craft::ability(ManaCost { generic: 8, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, &ABILITY_2_MATERIALS, "Craft with nonland with an activated ability {8}{U}")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); planned_value!("static change GainAbilitiesOf", () => chars.gain_abilities_of(AbilitiesOf { objects: { let ids: Vec<ObjectId> = q.exiled_with(q.linked_to(source)).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>(); ids }, kinds: &[CheckHasable::ActivatedAbility], once_each_turn: true, spend_as: None })); }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AbilityActivated { source, ability, player, .. } = event { let id = *source; let def = q.activated_ability(*ability); let subject = Target::Player(*player); let serial = match cantrip_mechanics::triggers::activated_ability_on_stack(q, event) { Some(Target::Ability(serial)) => serial, _ => 0 }; let data = [id.0, serial]; if def.is_some() && Some(asker.controller) == Some(*player) && !(def.is_some_and(|d| d.is_mana_ability())) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let copier_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let t = Target::Ability(item.data[1]); g.copy_ability(t, copier_0, true)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Locus of Enlightenment", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(2)), power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ABILITY_ACTIVATED.0), ability_1_event), ability_1, "Whenever you activate an ability that isn't a mana ability, copy it. You may choose new targets for the copy.").targets(&[])], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_0_apply_0, "Locus of Enlightenment has each activated ability of the exiled cards used to craft it. You may activate each of those abilities only once each turn.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
