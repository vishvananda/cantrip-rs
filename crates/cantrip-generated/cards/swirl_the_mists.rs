// Swirl the Mists
// status: planned
// requires: static change ReplaceWord
// As this enchantment enters, choose a color word.
// All instances of color words in the text of spells and permanents are changed to the chosen color word.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = entering.controller; let Answer::Colors(colors_0) = g.ask(player_0, RequestKind::ChooseColors { source: entering.object, choices: ColorSet(31), count: 1, purpose: ColorUse::Any })? else { return Ok(()) }; let chosen_0 = colors_0[0];
entering.chosen_color = Some(chosen_0);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (true || (q.state().obj(affected).became.is_none() && q.state().obj(affected).zone.kind() == ZoneKind::Stack) && true) }
planned_item!("static-change-replace-word", "(kind:Static,args:(kind:Ordinary,affected:Objects((kind:Union,args:[(kind:Battlefield,args:(kind:Any)),(kind:Zone,args:(scope:Shared(Stack),kind:Spell,filter:(kind:Any)))])),changes:[(kind:ReplaceWord,args:(from:(kind:Vocabulary,args:(family:\"Color\",form:\"AnyColor\")),to:(kind:Vocabulary,args:(family:\"Color\",form:\"TheChosenColor\"))))]))");

pub static CARD: CardDef = CardDef { name: "Swirl the Mists", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
