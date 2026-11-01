// Copycrook
// status: ready
// You may have this creature enter as a copy of any creature on the battlefield, except it has "Whenever this creature attacks, it connives." (Draw a card, then discard a card. If you discarded a nonland card, put a +1/+1 counter on this creature.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn granted_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn granted_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_2: u32 = 1; if let Some(object) = ({ let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }).first().copied() { cantrip_mechanics::connive_n(g, object, asker.source, amount_2)?; }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); const ASK_0: Question = Question::new("CopyAsEnters", "enter as a copy?", "You choose what it copies next. If you don't, it enters as itself", "Choose a copy", "Enter as itself"); let candidates_0: Vec<ObjectId> = ({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| id != entering.object).collect(); if let Some(chosen_0) = cantrip_mechanics::copy::choose_copy_of_one_of(g, entering, &candidates_0, &ASK_0)? {
entering.copy = { let q: &dyn Query = g; Some(chosen_0).map(|original: ObjectId| q.copiable_values(original).with_ability(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) }; let picked_0 = vec![chosen_0];
}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Copycrook", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Shapeshifter,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), granted_1_event), granted_1, "Whenever this creature attacks, it connives.").targets(&[])], enters: None, modal: None, enchant: None, enters_as_copy: Some(ability_0_enters), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
