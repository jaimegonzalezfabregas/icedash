// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'main.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$Room {

 Object get field0;



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Room&&const DeepCollectionEquality().equals(other.field0, field0));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(field0));

@override
String toString() {
  return 'Room(field0: $field0)';
}


}

/// @nodoc
class $RoomCopyWith<$Res>  {
$RoomCopyWith(Room _, $Res Function(Room) __);
}


/// Adds pattern-matching-related methods to [Room].
extension RoomPatterns on Room {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( Room_Lobby value)?  lobby,TResult Function( Room_Trial value)?  trial,required TResult orElse(),}){
final _that = this;
switch (_that) {
case Room_Lobby() when lobby != null:
return lobby(_that);case Room_Trial() when trial != null:
return trial(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( Room_Lobby value)  lobby,required TResult Function( Room_Trial value)  trial,}){
final _that = this;
switch (_that) {
case Room_Lobby():
return lobby(_that);case Room_Trial():
return trial(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( Room_Lobby value)?  lobby,TResult? Function( Room_Trial value)?  trial,}){
final _that = this;
switch (_that) {
case Room_Lobby() when lobby != null:
return lobby(_that);case Room_Trial() when trial != null:
return trial(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( Board field0)?  lobby,TResult Function( Creature field0)?  trial,required TResult orElse(),}) {final _that = this;
switch (_that) {
case Room_Lobby() when lobby != null:
return lobby(_that.field0);case Room_Trial() when trial != null:
return trial(_that.field0);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( Board field0)  lobby,required TResult Function( Creature field0)  trial,}) {final _that = this;
switch (_that) {
case Room_Lobby():
return lobby(_that.field0);case Room_Trial():
return trial(_that.field0);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( Board field0)?  lobby,TResult? Function( Creature field0)?  trial,}) {final _that = this;
switch (_that) {
case Room_Lobby() when lobby != null:
return lobby(_that.field0);case Room_Trial() when trial != null:
return trial(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class Room_Lobby extends Room {
  const Room_Lobby(this.field0): super._();
  

@override final  Board field0;

/// Create a copy of Room
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Room_LobbyCopyWith<Room_Lobby> get copyWith => _$Room_LobbyCopyWithImpl<Room_Lobby>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Room_Lobby&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Room.lobby(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Room_LobbyCopyWith<$Res> implements $RoomCopyWith<$Res> {
  factory $Room_LobbyCopyWith(Room_Lobby value, $Res Function(Room_Lobby) _then) = _$Room_LobbyCopyWithImpl;
@useResult
$Res call({
 Board field0
});




}
/// @nodoc
class _$Room_LobbyCopyWithImpl<$Res>
    implements $Room_LobbyCopyWith<$Res> {
  _$Room_LobbyCopyWithImpl(this._self, this._then);

  final Room_Lobby _self;
  final $Res Function(Room_Lobby) _then;

/// Create a copy of Room
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Room_Lobby(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as Board,
  ));
}


}

/// @nodoc


class Room_Trial extends Room {
  const Room_Trial(this.field0): super._();
  

@override final  Creature field0;

/// Create a copy of Room
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Room_TrialCopyWith<Room_Trial> get copyWith => _$Room_TrialCopyWithImpl<Room_Trial>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Room_Trial&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Room.trial(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Room_TrialCopyWith<$Res> implements $RoomCopyWith<$Res> {
  factory $Room_TrialCopyWith(Room_Trial value, $Res Function(Room_Trial) _then) = _$Room_TrialCopyWithImpl;
@useResult
$Res call({
 Creature field0
});




}
/// @nodoc
class _$Room_TrialCopyWithImpl<$Res>
    implements $Room_TrialCopyWith<$Res> {
  _$Room_TrialCopyWithImpl(this._self, this._then);

  final Room_Trial _self;
  final $Res Function(Room_Trial) _then;

/// Create a copy of Room
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Room_Trial(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as Creature,
  ));
}


}

/// @nodoc
mixin _$Tile {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Tile);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'Tile()';
}


}

/// @nodoc
class $TileCopyWith<$Res>  {
$TileCopyWith(Tile _, $Res Function(Tile) __);
}


/// Adds pattern-matching-related methods to [Tile].
extension TilePatterns on Tile {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( Tile_Entrance value)?  entrance,TResult Function( Tile_Gate value)?  gate,TResult Function( Tile_Wall value)?  wall,TResult Function( Tile_Ice value)?  ice,TResult Function( Tile_ThinIce value)?  thinIce,TResult Function( Tile_WeakBox value)?  weakBox,TResult Function( Tile_Outside value)?  outside,required TResult orElse(),}){
final _that = this;
switch (_that) {
case Tile_Entrance() when entrance != null:
return entrance(_that);case Tile_Gate() when gate != null:
return gate(_that);case Tile_Wall() when wall != null:
return wall(_that);case Tile_Ice() when ice != null:
return ice(_that);case Tile_ThinIce() when thinIce != null:
return thinIce(_that);case Tile_WeakBox() when weakBox != null:
return weakBox(_that);case Tile_Outside() when outside != null:
return outside(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( Tile_Entrance value)  entrance,required TResult Function( Tile_Gate value)  gate,required TResult Function( Tile_Wall value)  wall,required TResult Function( Tile_Ice value)  ice,required TResult Function( Tile_ThinIce value)  thinIce,required TResult Function( Tile_WeakBox value)  weakBox,required TResult Function( Tile_Outside value)  outside,}){
final _that = this;
switch (_that) {
case Tile_Entrance():
return entrance(_that);case Tile_Gate():
return gate(_that);case Tile_Wall():
return wall(_that);case Tile_Ice():
return ice(_that);case Tile_ThinIce():
return thinIce(_that);case Tile_WeakBox():
return weakBox(_that);case Tile_Outside():
return outside(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( Tile_Entrance value)?  entrance,TResult? Function( Tile_Gate value)?  gate,TResult? Function( Tile_Wall value)?  wall,TResult? Function( Tile_Ice value)?  ice,TResult? Function( Tile_ThinIce value)?  thinIce,TResult? Function( Tile_WeakBox value)?  weakBox,TResult? Function( Tile_Outside value)?  outside,}){
final _that = this;
switch (_that) {
case Tile_Entrance() when entrance != null:
return entrance(_that);case Tile_Gate() when gate != null:
return gate(_that);case Tile_Wall() when wall != null:
return wall(_that);case Tile_Ice() when ice != null:
return ice(_that);case Tile_ThinIce() when thinIce != null:
return thinIce(_that);case Tile_WeakBox() when weakBox != null:
return weakBox(_that);case Tile_Outside() when outside != null:
return outside(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  entrance,TResult Function()?  gate,TResult Function()?  wall,TResult Function()?  ice,TResult Function( int field0)?  thinIce,TResult Function( int field0)?  weakBox,TResult Function()?  outside,required TResult orElse(),}) {final _that = this;
switch (_that) {
case Tile_Entrance() when entrance != null:
return entrance();case Tile_Gate() when gate != null:
return gate();case Tile_Wall() when wall != null:
return wall();case Tile_Ice() when ice != null:
return ice();case Tile_ThinIce() when thinIce != null:
return thinIce(_that.field0);case Tile_WeakBox() when weakBox != null:
return weakBox(_that.field0);case Tile_Outside() when outside != null:
return outside();case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  entrance,required TResult Function()  gate,required TResult Function()  wall,required TResult Function()  ice,required TResult Function( int field0)  thinIce,required TResult Function( int field0)  weakBox,required TResult Function()  outside,}) {final _that = this;
switch (_that) {
case Tile_Entrance():
return entrance();case Tile_Gate():
return gate();case Tile_Wall():
return wall();case Tile_Ice():
return ice();case Tile_ThinIce():
return thinIce(_that.field0);case Tile_WeakBox():
return weakBox(_that.field0);case Tile_Outside():
return outside();}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  entrance,TResult? Function()?  gate,TResult? Function()?  wall,TResult? Function()?  ice,TResult? Function( int field0)?  thinIce,TResult? Function( int field0)?  weakBox,TResult? Function()?  outside,}) {final _that = this;
switch (_that) {
case Tile_Entrance() when entrance != null:
return entrance();case Tile_Gate() when gate != null:
return gate();case Tile_Wall() when wall != null:
return wall();case Tile_Ice() when ice != null:
return ice();case Tile_ThinIce() when thinIce != null:
return thinIce(_that.field0);case Tile_WeakBox() when weakBox != null:
return weakBox(_that.field0);case Tile_Outside() when outside != null:
return outside();case _:
  return null;

}
}

}

/// @nodoc


class Tile_Entrance extends Tile {
  const Tile_Entrance(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Tile_Entrance);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'Tile.entrance()';
}


}




/// @nodoc


class Tile_Gate extends Tile {
  const Tile_Gate(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Tile_Gate);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'Tile.gate()';
}


}




/// @nodoc


class Tile_Wall extends Tile {
  const Tile_Wall(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Tile_Wall);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'Tile.wall()';
}


}




/// @nodoc


class Tile_Ice extends Tile {
  const Tile_Ice(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Tile_Ice);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'Tile.ice()';
}


}




/// @nodoc


class Tile_ThinIce extends Tile {
  const Tile_ThinIce(this.field0): super._();
  

 final  int field0;

/// Create a copy of Tile
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Tile_ThinIceCopyWith<Tile_ThinIce> get copyWith => _$Tile_ThinIceCopyWithImpl<Tile_ThinIce>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Tile_ThinIce&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Tile.thinIce(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Tile_ThinIceCopyWith<$Res> implements $TileCopyWith<$Res> {
  factory $Tile_ThinIceCopyWith(Tile_ThinIce value, $Res Function(Tile_ThinIce) _then) = _$Tile_ThinIceCopyWithImpl;
@useResult
$Res call({
 int field0
});




}
/// @nodoc
class _$Tile_ThinIceCopyWithImpl<$Res>
    implements $Tile_ThinIceCopyWith<$Res> {
  _$Tile_ThinIceCopyWithImpl(this._self, this._then);

  final Tile_ThinIce _self;
  final $Res Function(Tile_ThinIce) _then;

/// Create a copy of Tile
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Tile_ThinIce(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

/// @nodoc


class Tile_WeakBox extends Tile {
  const Tile_WeakBox(this.field0): super._();
  

 final  int field0;

/// Create a copy of Tile
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Tile_WeakBoxCopyWith<Tile_WeakBox> get copyWith => _$Tile_WeakBoxCopyWithImpl<Tile_WeakBox>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Tile_WeakBox&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Tile.weakBox(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Tile_WeakBoxCopyWith<$Res> implements $TileCopyWith<$Res> {
  factory $Tile_WeakBoxCopyWith(Tile_WeakBox value, $Res Function(Tile_WeakBox) _then) = _$Tile_WeakBoxCopyWithImpl;
@useResult
$Res call({
 int field0
});




}
/// @nodoc
class _$Tile_WeakBoxCopyWithImpl<$Res>
    implements $Tile_WeakBoxCopyWith<$Res> {
  _$Tile_WeakBoxCopyWithImpl(this._self, this._then);

  final Tile_WeakBox _self;
  final $Res Function(Tile_WeakBox) _then;

/// Create a copy of Tile
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Tile_WeakBox(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

/// @nodoc


class Tile_Outside extends Tile {
  const Tile_Outside(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Tile_Outside);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'Tile.outside()';
}


}




// dart format on
