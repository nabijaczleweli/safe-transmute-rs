(function() {var implementors = {};
implementors["safe_transmute"] = [{"text":"impl Unpin for GuardError","synthetic":true,"types":[]},{"text":"impl&lt;'a, S, T&gt; Unpin for UnalignedError&lt;'a, S, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;S, T&gt; Unpin for IncompatibleVecTargetError&lt;S, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, S, T&gt; Unpin for Error&lt;'a, S, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Unpin,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Unpin for ErrorReason","synthetic":true,"types":[]},{"text":"impl Unpin for SingleValueGuard","synthetic":true,"types":[]},{"text":"impl Unpin for PedanticGuard","synthetic":true,"types":[]},{"text":"impl Unpin for AllOrNothingGuard","synthetic":true,"types":[]},{"text":"impl Unpin for SingleManyGuard","synthetic":true,"types":[]},{"text":"impl Unpin for PermissiveGuard","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()