LJ F@scripts/game/entity_system/systems/beam/beam_system_client.client.lua   lx*7  '   T47 '   T% H T`7 '   T7 '   T% H TU7 '   T% H TN% H TK7 '   T%	 H TD7 '   T%
 H T=7 '   T% H T6% H T37 '   T% H T,7 '   T7 '   T% H T!7 '   T% H T% H T7 '   T% H T7 '   T% H T	7 '   T% H T% H G  beam_explode_lifebeam_explode_life_waterbeam_explode_life_coldbeam_explode_life_fire beam_explode_life_lightning$beam_explode_life_lightningcold$beam_explode_life_lightningfirebeam_explode_life_steambeam_explode_arcanebeam_explode_arcane_water
waterbeam_explode_arcane_coldbeam_explode_arcane_fire"beam_explode_arcane_lightning&beam_explode_arcane_lightningcold	cold&beam_explode_arcane_lightningfire	firelightningbeam_explode_arcane_steam
steamarcane

    !!!""""###$$$$%%%''*elements  m È   3L0
7  '   T7'    T(7 '   T7 '    T 7 '   T7'    T7 '   T7'    T7 '   T7'    T7 '   T7'   T) H G  	fire	coldlightning
water	lifearcane
elements1  4elements2  4 ¸   '?<)  7  '   T% H T7 '   T% H T7 '   T% H T7 '   T% H T	7 '   T%	 H T%
 H G  7content/particles/spells/beams/beam_life_knockdown9content/particles/spells/beams/beam_arcane_knockdownarcane8content/particles/spells/beams/beam_water_knockdown
water7content/particles/spells/beams/beam_fire_knockdown	fire7content/particles/spells/beams/beam_cold_knockdown	cold8content/particles/spells/beams/beam_steam_knockdown
steam			



elements  (elem_name & ¨ 3M[4  7  %  >  7 % % >  7 % % >2  :	 2  :
 2  : '  :   7 >2  : 2  : 2  : 4 77 >: ' : 2 +  7>;+  7> <  : G   Ànewdeleted_beam_dataupdate_index
world
Worldphysics_worldbeams_to_removebeams_to_add
beamsregister_extensions%reset_nr_of_destroyed_beams_time)nr_of_destroyed_beams_per_owner_unit authorative_hit_per_shooterbeam_effectsrpc_from_server_beam_hit!rpc_from_server_beam_exploderegister_network_messagesbeam_destroyedbeam_createdregister_eventsbeam_system	initEntitySystemÀ

pdArray self  4context  4 :   v7 :  G  effect_managerself  context     Nz+7   TG  7   T'   :   7  >'  ' IQ67  T	T7	 6		 	 T
'	  7
  	9
Kñ) : 7 7   97  T	7)  :7)  :7)  :	7
  T	7
)  :7
)  :7
)  :	7 7>7
  T  7 7>)  :G  beam_destroyed	stopeffect
rightreflectorchild_reflected
child	leftbeams_to_remove)nr_of_destroyed_beams_per_owner_unitowner_unitfind_recursive_parentsdebug_beam_counterdestroyed



   !!!"""$$$$'''(((())+self  Obeam_data  Oparents @  i parent_beam owner_unit amount  á
 ¸¨H+  ) : 4 7+  4 7+  7+  7 >:   T
+  )  :+  )  :+  )  :	G  4
 7+ 7>+  7  T) T) +  7+  '  :*   '	 
 ' I	A6 8 8  T4 7 >  T)  
  T!+  7 T
  T T  T4 7 % >  T4 7 % >  T'   T88    
  T+  7 T

  T T88    K	¿
  T	 T	  T	'	  :	4	 7		
 >		 +	  4
 7

 >
:
	+	  :	+	  4
 7

 >
:
	+	  :		+	  :	T		+	  )
  :
	+	  )
  :
	+	  )
  :
		G  À Àraycast_distas_tableVector3AuxnormalizeVector3zvelocityget_data	UnitplayerextensionEntityAuxowner_unit	unit
Actorlocal_pre_hit_time
world	time
Worldraycast_hit_unitraycast_hit_normalraycast_hit_positiongrowthmin	mathlengthbeam_max_lengthSpellSettingsraycast_waiting			


          !!!!!!!!!"""""#####$%&'()//////////012345::::::;;<<<<<======>>??????@@AAADDDEEEFFFHbeam_data self hits  beam_max_length is_pre_hit_time #vlength tclosest_position pclosest_unit  pclosest_distance  pclosest_normal  pnum_hits oB B Bi @hit ?hit_distance =hit_actor <hit_unit 	3player_velocity hit_position hit_normal hit_position hit_normal  ¾ !R§l1  % 4 77  % % %	 %
	  >:7
   T'   :
 '  :2  :4 77 >4 77 >:7
  T77) 97  T4 7777>7  T4 7777>'  :4 7 7 7 7 	 >:7 7   797  7    90  G  beams_to_addbeam_effectsevent_delegateunit_spawnereffect_managerBeamEffecteffectgrowth
right	join
table	leftowner_unitserver_peer_id	pingNetwork
world	time
Worldlocal_pre_hit_timeownersraycast_hit_timerdebug_beam_countercollision_filter	both
typesallphysics_worldmake_raycastPhysicsWorldraycast_obbeam_query_with_players µæÌ³æýILMMMNOPPQQMQSSSSSSVVWWYYYYYYYYYYY[[[\\\\___``````cccddddddggiiiiiiiijjjjjjkkkkkllself  Sbeam_data  Sraycast_callback Qquery_filter P   Is7  7  :   :  7   T7 7  T4 7% > 7>  7	 >  7
 >  7  >  7  >  7	 >  7 >  7	 >  7 >  7 >  7  >  7  >  7 >  7 >  7  >  7  >G  handle_debug_testing$handle_too_many_destroyed_beamsdispatch_raycastsdebug_draw_beamsupdate_beam_effectsdeal_beam_damageupdate_authorative_beamsupdate_reflected_beamsbuild_intersectionsupdate_beams_recursivelypre_update_beamsvalidate_beams!handle_beam_adds_and_removes
resetdrawer_beamsdrawerpdDebugown_peer_idserver_peer_idSERVERprev_frame_indexupdate_index


self  Jdt  Jcontext  Jprev_frame_index H ò   =z³ )  4  7 >D$'
  T   T4 7	 %
 4 7) >7 '	  9	T'  T  T  T4 7	 %
 4 7) >7 '	  9	BNÚ7   T'  : 7 '  T'  : 2  : T7 : G  %reset_nr_of_destroyed_beams_timeknockdown	CSMEcharacterset_inputEntityAux)nr_of_destroyed_beams_per_owner_unit
pairs



 self  >dt  >initiator_unit <' ' 'owner_unit $amount  $     ÕG  self  dt   Ö   <Ù
4  7  >  7 >7 '  ' I6)  :Kü)  : G  raycast_ob
beams!handle_beam_adds_and_removesdestroyEntitySystem		
self  beams 	  i beam_data    .å' 7   ' I7  67 7   97  )  9Kõ' 7  ' I7 6' 7  ' I7
 6
	

 T)  :4 77 	 >TKó7 )  9KèG  remove
tableraycast_obbeams_to_remove
beamsbeams_to_add					

	self  /  i 
beam_data   i beam_data   j beam_data_cmp 
 ô   cú) :  4 7' 7  ' IQ7 67	 		:	)	  :	)	  :	)	  :	4		 7



 >	KïG  raycast_waitingassertrotationposition_endposition_startbeam_velocitygrowth
beamsbeam_max_lengthSpellSettings)validation_beams_are_done_processing		



self  dt  spellsettings beam_max_length   i beam_data  É  P2  ' 7   ' IQ7  6	  7 
  >  T	  9KòH is_recursive_child
beamsself  beam_data  parents   i beam_data_tmp 
    57   T
7  T) H   7 7  @ ) H is_recursive_child
child

self  beam_data  child_beam_data   ú  @Õ£»2    7  >7  '  ' IRQP7 67		  T
TJ4
 7

	 >
 
 T
  7
  >
T?4
 7

	 '  >
:
4
 7
	
	 '  >
:
7
4 7	77>7
:

7
4 77>7 

:
7
  
 T7
 6
	
 
 T7
 6
	
7

:
7
 6
	
7


 7

>
:
7
 6
	
7

:
7
 6
	
7

:
T
)
  :
)
  :

 
 
9
K®2  : 2  : 2   '   TQQPQN 67
  T	7		  T	)	 T
)	 
  T
7

  T

	 T)
 T)
 4 7> TTá 
 T7 T7 7  T	7  T77  T)
 T)
  
 T®)  ' 7  ' I7 6 T TKù  T
4! 7"7  >4! 7"7  >4# 7 T7  T) T) >7 7  T' 77  T' 77$  T7 7$  TTT!4 77>% T4 77>% T4 7 7>% T4 7 7>% T	  7 7>  7 7 >Ts4& 7'77777 77 7>  T  7  >Ta 974( 7)77 >:7 4( 7)7 7 >:7:7 :7) :*7 ) :*7 7(  T' 4 777>4 77 7>4( 7+>4 7, 4( 7-> =::74 77>7 :T 	 TW7  T7.  T4 7 >  T	  7  >)  :/)  :0T71  T4& 771>:4& 772>4 77>4& 73  >4 7, 4( 7-> =:7:74 77>7 :7  T774:4( 75  >46 77 T  9TÊ~ 9TÇ~TÆ~TÅ~  TÃ~ 
 TÁ~7 7   48 %9   T7:  T%; >97 7   9T­~  7< >  7  >'  '	 I6
7=  T7$  T  7>  >Kô) :?   7< >  7  >G  )validation_beams_are_done_processingexplode_beamtraversed!handle_beam_adds_and_removesnilDEBUGNAMEchild %qsprintfbeam_reflection_theta_maxSpellSettingsdotraycast_distreflectionraycast_hit_normalraycast_hit_positionchild_reflectedreflectorraycast_hit_unitup	looknormalizeintersectsdistanceVector3segment_intersection_XYVector3Auxvector3destroyedassertremove
table
right	leftquaternion	typereflector_source
childdebug_childsdebug_stuffsectionhit_sectionhit_unitserver_hit_unit
unboxhit_offsetauthorative_length authorative_hit_per_shooterlengthforwardQuaternionposition_endowner_staff_nodeowner_staff_unitzworld_positionposition_startlocal_rotationrotationbeam_destroyed
alive	Unitowner_unit
beamsvalidate_beamsµæÌ³æýþÿÿÿ		
"""%%&&(*+,,,,-.0111111112222222222555556::;;;;;;;==========??ABBBBBCCCCDEBHHIIIIIJJJJJMMMMMMMMMMMOOOOPSSSSTXXXXXXXXYYZZZZZZZZZZZZZZZZZZZZZZZZ[[[[\\\\]``aabbccdd`ffgggggijllllllllmmmmmmmmnnoopppqqqssssstwwwwwxxxxxzzzz{{{{{{{|~ ¡¢¤¥¦¦¦¦§§§§§§§§§§§§§§¨¨¨¨¨«­­­¯¯¯±±±±²³³³³³³´´´´±¸¸¹¹¹ººº»self  Ödt  Öbeams_to_update Ônum_beams ÏS S Si Qbeam_data Nowner_unit Mbeams_to_explode Sônum_beams_to_update ói òbeam_data Ìchild Ëis_child_reflected Ãcan_process_child 
¹debug_remove_index ¬  d bp # bp  intersection_point 6Iua  Iub  Ibp ) forward_left forward_right new_direction rotation hit_unit Rhit_normal 3forward /new_direction *rotation #beam_reflect_dot /  i beam_data 
 ë   )¢à'  ' I$Q	"6		 T
T	7
 	 
 TT	7
	
 T

7
	
 T) : 	T) : 	 
 7
	
 T

7
	
 T) : 	T) : 	 
 KÜG  ub
rightua	leftcleared	

self  *intersections  *intersection  *beam_data  *u_cmp  *% % %i #intersection_other !u_cmp_other 	u_cmp_other 	 D    7  7   T) T) H 
u_sum        i1  	i2  	 Ï4C°Ãñ2  7   '  ' Iy7  677	
  ' I
pQn7  6777 TTf  T7 7 ' * 	 T	 ' ' T17  TW7  TTST4  > T4 	 > T  7	  >TCT4  > T4  > T  7	  >T34
 7 	   >     TT&7 T7	 T' 3 :::7 :7 :7 7 :4 % 4 77>4 77> =:  9K
K  7 >2  2  '  ' I96	7
	  ' I,Q*674 7
  >(  T ) :	) :7	) 97	) 977	 T77	 T7) 977	 T77	 T7) 9KÔ7	  T  9	KÇ4  >D  7
	  >
7


  T
7
)  :
7
)  :
BNñ   '  T'   7 >  7 >4 7  1! >  7 >)   TQ) '  '	 I6
7"  T  7#   77>7"  T  7#   77>Kê '  T	ÞQ	b6	7
"	 
 T\) 7
	7



  T
7
	7

7	7
 T
  7
	 7	7>
7
	)  :
7
	)  :
7
	)  :
7
	)  :
  7
 >
  7
 >
T
37
	7



  T
7
	7



  T
  7
	 7	7>
7
	)  :
7
	)  :
  7
 >
  7
 >
T
7
	7



  T
7
	7



  T
  7
	 7	7>
7
	)  :
7
	)  :
  7
 >
  7
 >
6
9
)
  9
 T
 T	Tz  7 >  7 >'  '	 IF6
77$
  T	7)  :%7)  :&7)  :$77$
  T	7)  :%7)  :&7)  :$77
  T  7	 77>7)  :7)  :  7 >  7 >77
  T  7	 77>7)  :7)  :  7 >  7 >Kº  7 >  7 >2  '  '	 I6
77  T77  T  9Kó   7 >  7 >'  '	 Ie6
4' 7(77)>4' 7(77)>4 7*  >4+ 7, TO  7	 7>  7	 7>7)  :7)  :7)  :7)  :  7 >  7 >7
  T77-  T  7. 7>7
  T77-  T  7. 7>77/  T77/  T70 41 72 %3 74' 74> =75  76%7 )  7>7) :/7) :/K2  '  '	 I6
77  T77  T  9Kó  ' 	 '
 IÈQÆ677  T77  TT¼77  T4 %8 4 77>4 77>4 777> =:T¦4' 7(77)>4' 7(77)>4 7*  >4+ 7,  T) T) +  779779>  T  T<  7. 74 7777> =  7. 74 7777> =  7 >  7 >77/  T77/  T^70 41 72 %3 74' 74> =75  76%7 )  7>7) :/7) :/TF4 7:>4' 7; 4 7<> =4 7=4 7>77?>77?>4@  >3A 7:7::9:?:)7:7:  7B  >77  T77  T' 7:7:4 %8 4 77>4 77>4 777> =:  7 >K8	  7 >	  7 >G  Àbeam_created length  ElementQueue_queue_to_tableelement_queue
clonearray_concatup	looknormalizeelements%s+%s->%s"play_spell_beam_merge_collideplay_soundeffect_manageridentity;content/particles/spells/beams/beams_crossed_explosioncreate_particles
World
worldplayed_crossed_explosionexplode_beamtraversedbeam_reflection_theta_maxSpellSettingsdotrotationforwardQuaternionraycast_hit_normalraycast_hit_positionraycast_hit_unitclear_worse_intersectionscleared 	sort
table!handle_beam_adds_and_removeschild_reflectedreflector
pairsdistanceVector3validate_beamstostring
%s+%ssprintfDEBUGNAME
u_sumubuaintersection_point
right	left  lengthsegment_intersection_XYVector3Auxbeam_destroyedvector3	typedestroyeddebug_stuffdebug_childs
childposition_endposition_start
beams µæÌ³æýþÿÿÿ   	 	 	 	 
 
                        ! " # # ' ' ' ' ' ' * * + + + + + + + + + + . . . . / / 0 0 0 0 0 0 0 0 0 0 3 3 3 3 4 7 7 7 8 9 : 7 : : : = = = ? ? ? ? ? ? @ C E F G H H H I I I J J J J J J M M M M M M M M M M M M N N N  	 R R R U V W W W W X Y [ [ [ [ [ \ ] _ _ _ _ _ _ _ _ ` ` a a c c c d d d f f f f f f f f g g g i i i i i i i i j j j [ o o o p p p W t t t t u u u u v v v w w w x x x t t | } ~ ~ ~ ~                                                                                    ¡ ¡ ¡ £ £ £ ¤ ¤ ¤ ¤ ¥ ¥ ¥ ¥ ¥ ¥ ¥ ¥ ¦ ¦ ¦ ¦ ¦ § § § ¨ ¨ ¨ ª ª ª « « « « ¬ ¬ ¬ ¬ ¬ ¬ ¬ ¬ ­ ­ ­ ­ ­ ® ® ® ¯ ¯ ¯ ± ± ± ² ² ² µ µ ¶ ¶ ¸ ¸ º » ¼ ¿ ¿ ¿ À À À Â Â Â Â Ã Å Å Å Å Æ Æ Æ Ç Ç Ç È È È Ë Ë Ë Ë Ì Ì Ì Í Í Í Î Î Î Ñ Ñ Ñ Ñ Ò Ò Ò Ò Ò Ó Ó Ó Ô Ô Ô × × × Ø Ø Ø Ú Ú Ú Ú Û Û Û Û Û Ü Ü Ü Ý Ý Ý à à à á á á Â ä ä ä å å å ç è è è è é ê ê ê ê ê ê ê ê ë ë ë è î ð ð ð ñ ñ ñ ó ó ó ó ô ö ö ö ö ö ÷ ÷ ÷ ÷ ÷ ø ø ø ø ø ù ù ù ù ù ú ú ú ú û û û û ý ý ý þ þ þ ÿ ÿ ÿ    				ó    #%&&&&&'((((((((*--../////////////////58888899999:::::;;;;;;;;<<<<<<====???????????@@@@@@@@@@@BBBCCCEEEEEEEEFGGGGGGGGGHHHHHHHIIIJJJKOOOOPPPPPPPSSSSSSSSSSTTTUWWXXZ[]^^__aaaaccccccccdffgghhhhhhhhhhhhhhhhhkkk&ooopppqcontains_opposite_elements self  ±intersections ¯num_beams ­z z zi xbeam_data vbeam_data_start ubeam_data_end tq q qj obeam_data_other lbeam_data_other_start kbeam_data_other_end jdebug_childs debug_stuff  bp  intersection_point `ua  `ub  `bp A intersection intersections_to_keep ¬beams_to_remove «: : :i 8intersection 7intersection_start 6- - -j +intersection_other )intersection_start_other (3  beam_data _  intersections_old_debug Úbp  cleared_found Å  i intersection num_intersections fi eintersection `kG G Gi Eintersection DO  i intersection f f fi dintersection cforward_left ^forward_right Ybeam_collide_dot Tworld >  i intersection num_intersections ÓÉ É Éi Çintersection Åchild ºforward_left ¡forward_right beam_collide_dot frontal_collision has_opposite_elements world )new_direction Brotation ;element_queue 
1elements .beam_data "apan   ½  )`ó' 7   ' IQ7  67  T7  TT) :4 77>7 4 77		
  4
 7 >Kã  7 >  7 >G  validate_beams!handle_beam_adds_and_removesbeam_max_lengthSpellSettingsraycast_ob	castRaycastposition_startforwardQuaternionraycast_waitingrotationdestroyed
beamsself  *  i beam_data dir raycast_start 	 À
  #ØK  7  >' 7  ' IQ7 67  TT4 7> TTy774 7				 >) )	 7



  T4 7
 >  T4 7
 % >  T) 7
 T)	 T)	  	 T7
  T4 7  T) T) >4 777 T) T) >  7 7>)  :)  :  T9 	 T74 7
  T) T) >:
4 77>4 77>4 7  >4 7 4 7> =3 ::
7:7:7 : :7:74 7 > :::  7!  >Ky  7" >  7  >G  !handle_beam_adds_and_removesbeam_createdowner_peer_idelement_queueelements	left length upVector3	lookreflectionforwardQuaternionraycast_hit_normal
unboxVector3Aux
childbeam_destroyedreflector_sourcechild_reflectedreflectorget_data
alive	Unitraycast_hit_unitraycast_waitingassertposition_endposition_startquaternionrotation	typedestroyed
beamsvalidate_beams 




        !!!!!!!!!!####$$%%))))++++++++,....////00000111111135799::<<=>>????????BCDDDDIIIJJJKself    i beam_data beam_data_start xbeam_data_end whit_reflector rhit_same_reflector qhit_unit phit_normal F*forward_left &new_direction !rotation beam_data_child  Â ¤ÂÏD7   '  ' IQ7  67  TT7  T7  TT74 7	 >  TT 7	7
 
 TQ
7:7
 
 T7
:
7
 
 Tò7
:
T
ï)
  :
	)
  :

4
 7

 '  >
74  > T4 '  '  '  > 4 7 >  T'  4 7
 7>  T'  +    TL+   TTH74 7> T7  T7'  T7  T87   T57 6	  T14 777>7 7 :T$+  T4 7 >+  
: 	 T7  T774 7 >+  T
4 7 >4 7	 %  >KcG  À
À	ÀÀauthorative_rotationset_data	lookQuaternionchild_reflectedposition_startnormalize authorative_hit_per_shooterpeer_idNetworkowner_peer_idposition_enddistancelengthVector3vector3	typehit_offsetlocal_positionraycast_hit_normalraycast_hit_unit
childowner_unit
alive	Unitserver_hit_unit
right	leftauthorative_length
beamsµæÌ³¦þ						
     !!!!!!#######$$$$$$$$''''''(+++++++++++++----------......//0000144455555577:::::;;;<<<<<<<====>>>>>>DBEAM_POSITION_ADJUST_LIMIT PLAYER_HIT_RADIUS LIMIT_FRACTION ADJUST_ROTATION_MIN_DIST self  ¥num_beams ¢  i beam_data hit_unit child_data shooter_unit unit_pos jhit_offset ioffset_length Wunit_distance Odirection $length direction firing_rotation  é $ -ß¢	b4  7  2  '  ' IÐQ
Î6
	7
'  :
7
  T7

  T4 7 >  T
4 7 %	 >  T) T)   T7

:

T4 77

'  >:

4  74  74  74 77

4  7!' > 7 6	4 77 6	7 4  7 >:  TT
4 7 % >  TT
~4 7 %	 >  TT
v  TT
s:
7 TT
m7
7:
) 4 7>'  4 7
>D
4 7 >  T) T 9BNô	 T)   TT
N7 7

7
4 7>4  >D,'   T)76 78%  >' ! T	 7"8 ! %"# %#$ > T 7"8 %!% %"# %#$ >   7&  ! >7 &! " >   7    9 BNÒ4 7'   %# 3( :>4 7 %) >  T4 7*  8>K0  7+ >  7, >G  validate_beams!handle_beam_adds_and_removes&add_status_magnitude_by_extensionstatus  add_damagescale_amount
waterdamage	beamget_template_value
steambeam_damageget_variabledamage_types_beamelementsplayer_variable_managerowners
pairsalloc_tableFrameTablebeam_damage_intervalreflectorhas_databeam_thickness_shift_speed	lerpbeam_thicknessbeam_effectsbeam_thickness_timeminbeam_max_thicknessbeam_min_thicknessmax	mathraycast_hit_timerdamage_receiverextensionEntityAux
alive	Unitraycast_hit_unitserver_hit_unitdamage_interval
beamsSpellSettings 		""#'()))*,,,,.///01111222222346711::;>>?BCDEEEFFFFGGGHHIIIIIJKKLLLLLLLLLNNNNNNNNPPQQQQRRRRRTTTFFXXXXXXXXXYYYYYZZ[[[[[[```aaabself  àdt  àspellsettings Þbeams Ýnum_beams Üauthorative_sending ÛÑ Ñ Ñi Ïbeam_data Ídamage_interval Ìraycast_hit_unit Ævalid_unit ´beam_thickness deal_damage .hcasters enum_casters d  owner 
_  
player_variable_manager Mdamage_scale Lelements Kdamage H/ / /element ,amount  ,base_damage 'player_beam_damage_modfier "template_beam_damage_modifier !tweaked_damage calculated_damage status_ext  £ 	 ^ù	7   '  TQ6 7 >7  T69)  9 Tï Tí  7 >  7 >G  validate_beams!handle_beam_adds_and_removes	doneupdatebeam_effects		

self  dt  beam_effects num_beam_effects i beam_effect      	
G  self   4   
  T) ,  G  valid test   ¹  ~¢
) 1  7   T 7  T) T) > 7  T) T) > 7  T) T) > 7
  T
77 T77 T) T) > 7
  T77 T) T) > 7
  T77 T) T) > 7  T7
  T) T) > 7
  T7  T) T) > 7	
  T7  T) T) >  T  7
  >0  G  7  T  7 7>7  T  7 7>0  G  validate_beambeam_destroyedchild_reflectedreflector
right	left
childrotationposition_endposition_start)validation_beams_are_done_processing 								







self  beam_data  valid }check | ¡ 
  <³
' 7   ' I	Q7  67  7 	 >K÷G  validate_beamDEBUGNAME
beamsself  
 
 
i beam_data DEBUGNAME  ÿ  MÝ»
-) : ' 2 ;'   T+Q* 67
  T
77   T7) :  797
  T
77   T7) :  797
  TÞ77   TÚ7) :  79TÓ  7 >'  ' I6
	7
  T  7 
 >T)  : 
Kô  7 >  7 >G  !handle_beam_adds_and_removesbeam_destroyedowner_unitvalidate_beams
right	left
childtraversed							


!!!"""""$$+++,,,-self  Nbeam_data  Nauthorative_length  Nnum_beams_to_traverse Jbeams_to_traverse Htraverse_i Gbeam_data '.  i beam_data_traverse 
  'Ðë
74  % >7  7 >4 7 %	 4
 7

) >4	  	 +
  >	 +
  

 ' I	+  )  9K	ü4	
 +
  >	4
 7

 '  >
4 7>

' 7  '  ' I67 T  T'   T7 T T74 777>7:
TKè4 7 '  >+ 	 >4 7>4 74 7>
 >::	4 7   >7 4 7 '  >+ 	 >4  7!  %" >4  7#  
  >4  7$   4   '  > = 
4 7 %% %&  >G  ÀÀÀbeam_explodespellcastset_particles_variablecreate_particlesbeam_endfind_particles_variable
Worldlocal_rotation
worldadd_abilityelemental_makeupboxVector3Auxpositionalloc_tableFrameTablelocal_forwardowner_staff_nodeowner_staff_unitworld_positionzlengthowner_unit
beamsupVector3local_position	Unit ElementQueue_queue_to_table"ElementQueue_multipart_unpackknockdown	CSMEcharacterset_inputEntityAux	unitunit_storage2*****************rpc_from_server_beam_explodelog





#'''''((()))********+,,,,,,./////0001111112222222333333333335566666667TEMP_ELEMENT_QUEUE get_explode_type _get_knockdown_effect self  sender  owner_unit_go_id  authorative_length  packed_elements_part1  packed_elements_part2  owner_unit 	_TEMP_ELEMENT_QUEUE yunpacked_element_queue_n  y  i elements mposition_start 	ddistance cbeams bnum_beams a  i beam_data forward @ability_name =ability_data :world *rotation %effect_name "effect_variable_id effect_id position_end  ý  +¿¤7   7	 >  TG  7 )	  9	4 7	 >  T	  TG  7  	 7
 >4	 7		
 % >	 	 T

7
 3 :	4
  >:::9
G  sectionhit_unithit_offsetVector3Boxlength  playerextensionEntityAux
alive	Unit authorative_hit_per_shooter	unitunit_storage 					self  ,sender  ,shooting_unit_go_id  ,beam_section  ,authorative_length  ,hit_unit_go_id  ,hit_pos_offset  ,shooter_unit &hit_unit player_ext    
 ,÷µ"7 74 74 '  '  '  >)  4 7 >T
Q TT7 74 7    >  T	4 7	> T   A
N
æ  	 F lengthsegment_intersection_XYVector3Aux
beamsipairsVector3	huge	mathposition_endposition_start	!!!!self  -beam_data  -beam_start +beam_end *closest_distance (closest_point #closest_beam "  i beam_data_other  beam_other_start beam_other_end intersection_point ua  ub  distance  ¬  Arý Ù4   % > 1 1 1 4   T' 5 4   T' 5 % ) ' ' (  (	  
	( ' 4 4	 4
 >5	 4	 1 :4	 1 :4	 1 :4	 1 :4	 1 :4	 1 :4	 1 :4	 1 :4	 1 :4	 1 :4	 1  :4	 1" :!4	 1$ :#4	 1& :%4	 1( :'4	 1* :)4	 1, :+4	 1. :-4	 10 :/4	 12 :14	 14 :34	 16 :54	 18 :74	 1: :92  4	 1< :;4	 1> :=4	 1@ :?0  G   find_closest_intersection rpc_from_server_beam_hit !rpc_from_server_beam_explode explode_beam validate_beams validate_beam debug_draw_beams update_beam_effects deal_beam_damage update_authorative_beams update_reflected_beams dispatch_raycasts build_intersections clear_worse_intersections update_beams_recursively is_recursive_child find_recursive_parents pre_update_beams !handle_beam_adds_and_removes destroy handle_debug_testing $handle_too_many_destroyed_beams update beam_created beam_destroyed finalize_setup 	initEntitySystemClientBeamSystem
classABCDEFGHIJKLMNOPQRSTUVXYZglobal_alphabet_counterglobal_beam_counter   ,foundation/scripts/util/container/arrayrequireçÌ³³æÿçÌ³³æ¬ÿµæÌ³æý   . : K M M M M M N N N N N O P Q R S T U V W Y Y Y Y Y [ t [ v x v z ¥ z § § /3S3UWUYcYexezz¡£^£`~`ñó óMOO÷ùù1393;h;jk¢k¤³¤µ×µ××pdArray oget_explode_type ncontains_opposite_elements m_get_knockdown_effect lalphabet aDEBUG_BEAMS `AUTHORATIVE_THRESHOLD _BEAM_POSITION_ADJUST_LIMIT ^PLAYER_RADIUS ]LIMIT_FRACTION \PLAYER_HIT_RADIUS [ADJUST_ROTATION_MIN_DIST ZMAX_HIT_DISTANCE YTEMP_ELEMENT_QUEUE N  