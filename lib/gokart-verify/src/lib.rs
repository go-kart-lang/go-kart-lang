mod apply;
mod ctx;
mod err;
mod state;
mod verify;

pub use verify::verify;

// newtype Var = Var String
//   deriving (Eq, Show)
//   deriving newtype (Ord)
// newtype TypeVar = TypeVar String
//   deriving (Eq, Show)
//   deriving newtype (Ord)
// newtype TypeName = TypeName String
//   deriving (Eq, Show)
//   deriving newtype (Ord)

// data Const = ConstInt Int | ConstChar Char | ConstString String deriving (Show)

// data Expr
//   = VarExpr Var
//   | ConstExpr Const
//   | AbsExpr Var Expr
//   | AppExpr Expr Expr
//   | LetExpr Var Expr Expr
//   | FixExpr Var Var Expr
//   deriving (Show)

// data Type
//   = TTypeVar TypeVar
//   | FunctionType Type Type
//   | PrimType TypeName [Type]
//   deriving (Show)

// data TypeScheme = TypeScheme [TypeVar] Type deriving (Show)

// pprintExpr :: Expr -> String
// pprintExpr (VarExpr (Var x)) = x
// pprintExpr (ConstExpr (ConstInt x)) = show x
// pprintExpr (ConstExpr (ConstChar x)) = show x
// pprintExpr (ConstExpr (ConstString x)) = show x
// pprintExpr (AbsExpr (Var x) e) = "\\" <> x <> " -> " <> pprintExpr e
// pprintExpr (AppExpr e1 e2) = "(" <> pprintExpr e1 <> " " <> pprintExpr e2 <> ")"
// pprintExpr (LetExpr (Var x) e1 e2) = "let " <> x <> " = " <> pprintExpr e1 <> " in " <> pprintExpr e2
// pprintExpr (FixExpr (Var f) (Var x) e) = "fix (\\" <> f <> " " <> x <> " -> " <> pprintExpr e <> ")"

// pprintType :: Type -> String
// pprintType (TTypeVar (TypeVar s)) = s
// pprintType (FunctionType a b) = "(" <> pprintType a <> " -> " <> pprintType b <> ")"
// pprintType (PrimType (TypeName c) ts) = c <> " " <> unwords (map pprintType ts)

// pprintTypeScheme :: TypeScheme -> String
// pprintTypeScheme (TypeScheme ts t) = "forall " <> unwords (map (\(TypeVar s) -> s) ts) <> " . " <> pprintType t

// newtype AlgoState = AlgoState {freeTypeVariableCounter :: Int}

// type M = State.State AlgoState

// defaultAlgoState :: AlgoState
// defaultAlgoState = AlgoState 0

// type Ctx = Map.Map Var TypeScheme
// type Subst = Map.Map TypeVar Type

// lookupCtx :: Ctx -> Var -> TypeScheme
// lookupCtx ctx var = ctx Map.! var

// ctxPlus :: Ctx -> Var -> TypeScheme -> Ctx
// ctxPlus ctx k v = Map.insert k v ctx

// ctxFromList :: [(Var, TypeScheme)] -> Ctx
// ctxFromList = Map.fromList

// makeNewTypeVars :: Int -> M [TypeVar]
// makeNewTypeVars cnt = do
//   curCounter <- State.gets freeTypeVariableCounter
//   _ <- State.modify (\s -> s{freeTypeVariableCounter = freeTypeVariableCounter s + cnt})
//   pure $ map make [curCounter .. curCounter + cnt - 1]
//  where
//   make :: Int -> TypeVar
//   make i = TypeVar (newTypeVarPrefix <> show i)

//   newTypeVarPrefix :: String
//   newTypeVarPrefix = "_t#"

// makeNewTypeVar :: M TypeVar
// makeNewTypeVar = head <$> makeNewTypeVars 1

// ftvCtx :: Ctx -> Set.Set TypeVar
// ftvCtx ctx = foldr (Set.union . ftvTypeScheme) Set.empty (Map.elems ctx)

// ftvTypeScheme :: TypeScheme -> Set.Set TypeVar
// ftvTypeScheme (TypeScheme tyVars ty) = Set.difference (ftvType ty) (Set.fromList tyVars)

// ftvType :: Type -> Set.Set TypeVar
// ftvType (TTypeVar t) = Set.singleton t
// ftvType (FunctionType a b) = Set.union (ftvType a) (ftvType b)
// ftvType (PrimType _ types) = foldr (Set.union . ftvType) Set.empty types

// infixr 9 `combineSubst`

// -- equiavalent to s1 s2 (first apply s2, then s1)
// combineSubst :: Subst -> Subst -> Subst
// combineSubst s1 s2 = Map.union (Map.map (substType s1) s2) s1 -- union prefers first map

// substTypeScheme :: Subst -> TypeScheme -> M TypeScheme
// substTypeScheme subst (TypeScheme ts ty) = do
//   newTyVars <- makeNewTypeVars (length ts)
//   let newSubst = substFromList $ zip ts (map TTypeVar newTyVars)
//   pure $ TypeScheme newTyVars $ substType (subst `combineSubst` newSubst) ty

// substCtx :: Subst -> Ctx -> M Ctx
// substCtx subst = Monad.mapM (substTypeScheme subst)

// containsTyVar :: TypeVar -> Type -> Bool
// containsTyVar tv (TTypeVar tv2) = tv == tv2
// containsTyVar tv (FunctionType a b) = containsTyVar tv a || containsTyVar tv b
// containsTyVar tv (PrimType _ ts) = any (containsTyVar tv) ts

// unify :: Type -> Type -> Subst
// unify (TTypeVar a) (TTypeVar b) | a == b = idSubst
// unify (TTypeVar a) b
//   | containsTyVar a b = error "infinite type"
//   | otherwise = substFromList [(a, b)]
// unify a (TTypeVar b) = unify (TTypeVar b) a
// unify (FunctionType ty1 ty2) (FunctionType ty3 ty4) = List.foldl' f idSubst (zip [ty1, ty2] [ty3, ty4])
//  where
//   f :: Subst -> (Type, Type) -> Subst
//   f subst (t1, t2) = combineSubst (unify (substType subst t1) (substType subst t2)) subst
// unify (PrimType c1 ts1) (PrimType c2 ts2)
//   | c1 == c2 && length ts1 == length ts2 = List.foldl' f idSubst (zip ts1 ts2)
//  where
//   f :: Subst -> (Type, Type) -> Subst
//   f subst (t1, t2) = combineSubst (unify (substType subst t1) (substType subst t2)) subst
// unify c1 c2 = error $ "cannot unify " <> show c1 <> " and " <> show c2

// clos :: Ctx -> Type -> TypeScheme
// clos ctx ty = TypeScheme (Set.toList tyVars) ty
//  where
//   tyVars = Set.difference (ftvType ty) (ftvCtx ctx)

// getConstType :: Const -> Type
// getConstType (ConstInt _) = PrimType (TypeName "Int") []
// getConstType (ConstChar _) = PrimType (TypeName "Int") []
// getConstType (ConstString _) = PrimType (TypeName "String") []

// wAlgo' :: Ctx -> Expr -> M (Subst, Type)

// wAlgo' _ (ConstExpr c) = pure (idSubst, getConstType c)

// wAlgo' ctx (VarExpr var) = do
//   let TypeScheme typeVars ty = lookupCtx ctx var
//   newTypeVars <- makeNewTypeVars (length typeVars)
//   let subst = substFromList $ List.zip typeVars (map TTypeVar newTypeVars)
//   pure (idSubst, substType subst ty)

// wAlgo' ctx (AbsExpr x expr) = do
//   beta <- TTypeVar <$> makeNewTypeVar
//   let beta' = TypeScheme [] beta
//   (subst1, ty1) <- wAlgo' (ctxPlus ctx x beta') expr
//   pure (subst1, substType subst1 (FunctionType beta ty1))
//
// wAlgo' ctx (AppExpr e1 e2) = do
//   (subst1, t1) <- wAlgo' ctx e1
//   beta <- TTypeVar <$> makeNewTypeVar
//   (subst2, t2) <- substCtx subst1 ctx >>= \ctx' -> wAlgo' ctx' e2
//   let subst3 = unify (substType subst2 t1) (FunctionType t2 beta)
//   pure (subst3 `combineSubst` subst2 `combineSubst` subst1, substType subst3 beta)
// wAlgo' ctx (LetExpr x e1 e2) = do
//   (subst1, t1) <- wAlgo' ctx e1
//   ctx' <- substCtx subst1 ctx
//   (subst2, t2) <- wAlgo' (ctxPlus ctx' x (clos ctx' t1)) e2
//   pure (subst2 `combineSubst` subst1, t2)
// wAlgo' ctx (FixExpr f x e) = do
//   beta <- TTypeVar <$> makeNewTypeVar
//   let beta' = TypeScheme [] beta
//   (subst1, ty1) <- wAlgo' (ctxPlus ctx f beta') (AbsExpr x e)
//   let subst2 = unify (substType subst1 beta) ty1
//   pure (subst2 `combineSubst` subst1, substType subst2 ty1)

// wAlgo :: Ctx -> Expr -> (Subst, Type)
// wAlgo ctx expr = State.evalState (wAlgo' ctx expr) defaultAlgoState

// testSubst :: IO ()
// testSubst = do
//   let subst = substFromList [(TypeVar "a", TTypeVar (TypeVar "b"))]
//   let subst2 = substFromList [(TypeVar "b", TTypeVar (TypeVar "c"))]
//   print subst
//   print subst2
//   print $ combineSubst subst2 subst

// processExpr :: Ctx -> Expr -> IO ()
// processExpr ctx expr = do
//   let (subst, ty) = wAlgo ctx expr
//   putStrLn $
//     "Expr: "
//       <> pprintExpr expr
//       <> "\nResult: "
//       <> pprintType ty
//       <> "\nSubst: "
//   Monad.forM_ (Map.toList subst) $ \(TypeVar k, v) -> putStrLn $ " - " <> k <> ": " <> pprintType v

// main :: IO ()
// main = do
//   testSubst
//   putStrLn "############"
//   Monad.forM_ (Map.toList ctx) $ \(Var k, v) -> putStrLn $ k <> ": " <> pprintTypeScheme v
//   Monad.forM_ [expr0, expr1, expr2, expr3, expr4, expr5, expr6] $ \expr -> wrap (processExpr ctx expr)
//  where
//   makePrim s = PrimType (TypeName s) []
//   makeTy a = TTypeVar $ TypeVar a
//   funcs ts t = foldr FunctionType t ts
//   apps = List.foldl' AppExpr
//   appFunc s = AppExpr (VarExpr $ Var s)
//   vExpr = VarExpr . Var

//   wrap f = f >> putStrLn "------------"

//   ctx =
//     ctxFromList
//       [ (Var "odd", TypeScheme [] (FunctionType (makePrim "Int") (makePrim "Bool")))
//       , (Var "show", TypeScheme [] (FunctionType (makePrim "Int") (makePrim "String")))
//       , (Var "concat", TypeScheme [] $ funcs [makePrim "String", makePrim "String"] (makePrim "String"))
//       , (Var "ifthenelse", TypeScheme [TypeVar "a"] $ funcs [makePrim "Bool", makeTy "a", makeTy "a"] (makeTy "a"))
//       , (Var "False", TypeScheme [] $ makePrim "Bool")
//       , (Var "True", TypeScheme [] $ makePrim "Bool")
//       ]

//   expr0 = AppExpr (VarExpr $ Var "odd") (ConstExpr (ConstInt 1))

//   expr1 = LetExpr (Var "id") (AbsExpr (Var "x") (VarExpr $ Var "x")) (AppExpr (VarExpr $ Var "id") (ConstExpr (ConstInt 1)))

//   expr2 = AbsExpr (Var "x") (VarExpr $ Var "x")

//   expr3 = FixExpr (Var "rec") (Var "n") $ apps (vExpr "ifthenelse") [appFunc "odd" (vExpr "n"), ConstExpr (ConstString "1"), apps (vExpr "concat") [appFunc "show" (vExpr "n"), appFunc "rec" (vExpr "n")]]

//   expr4 = FixExpr (Var "rec") (Var "n") $ apps (vExpr "ifthenelse") [appFunc "odd" (vExpr "n")]

//   expr5 = apps (vExpr "ifthenelse") [vExpr "False", ConstExpr (ConstInt 1)]

//   expr6 = vExpr "False"
