;(function(){ 'use strict';

  var defaults = {
      container: false // can be a HTML or jQuery element or jQuery selector
    , viewLiveFramerate: 0 // set false to update graph pan only on drag end; set 0 to do it instantly; set a number (frames per second) to update not more than N times per second
    , dblClickDelay: 200 // milliseconds
    , removeCustomContainer: true // destroy the container specified by user on plugin destroy
    , rerenderDelay: 500 // ms to throttle rerender updates to the panzoom for performance
  };

  var debounce = (function(){
    /**
     * lodash 3.1.1 (Custom Build) <https://lodash.com/>
     * Build: `lodash modern modularize exports="npm" -o ./`
     * Copyright 2012-2015 The Dojo Foundation <http://dojofoundation.org/>
     * Based on Underscore.js 1.8.3 <http://underscorejs.org/LICENSE>
     * Copyright 2009-2015 Jeremy Ashkenas, DocumentCloud and Investigative Reporters & Editors
     * Available under MIT license <https://lodash.com/license>
     */
    /** Used as the `TypeError` message for "Functions" methods. */
    var FUNC_ERROR_TEXT = 'Expected a function';

    /* Native method references for those with the same name as other `lodash` methods. */
    var nativeMax = Math.max,
        nativeNow = Date.now;

    /**
     * Gets the number of milliseconds that have elapsed since the Unix epoch
     * (1 January 1970 00:00:00 UTC).
     *
     * @static
     * @memberOf _
     * @category Date
     * @example
     *
     * _.defer(function(stamp) {
     *   console.log(_.now() - stamp);
     * }, _.now());
     * // => logs the number of milliseconds it took for the deferred function to be invoked
     */
    var now = nativeNow || function() {
      return new Date().getTime();
    };

    /**
     * Creates a debounced function that delays invoking `func` until after `wait`
     * milliseconds have elapsed since the last time the debounced function was
     * invoked. The debounced function comes with a `cancel` method to cancel
     * delayed invocations. Provide an options object to indicate that `func`
     * should be invoked on the leading and/or trailing edge of the `wait` timeout.
     * Subsequent calls to the debounced function return the result of the last
     * `func` invocation.
     *
     * **Note:** If `leading` and `trailing` options are `true`, `func` is invoked
     * on the trailing edge of the timeout only if the the debounced function is
     * invoked more than once during the `wait` timeout.
     *
     * See [David Corbacho's article](http://drupalmotion.com/article/debounce-and-throttle-visual-explanation)
     * for details over the differences between `_.debounce` and `_.throttle`.
     *
     * @static
     * @memberOf _
     * @category Function
     * @param {Function} func The function to debounce.
     * @param {number} [wait=0] The number of milliseconds to delay.
     * @param {Object} [options] The options object.
     * @param {boolean} [options.leading=false] Specify invoking on the leading
     *  edge of the timeout.
     * @param {number} [options.maxWait] The maximum time `func` is allowed to be
     *  delayed before it's invoked.
     * @param {boolean} [options.trailing=true] Specify invoking on the trailing
     *  edge of the timeout.
     * @returns {Function} Returns the new debounced function.
     * @example
     *
     * // avoid costly calculations while the window size is in flux
     * jQuery(window).on('resize', _.debounce(calculateLayout, 150));
     *
     * // invoke `sendMail` when the click event is fired, debouncing subsequent calls
     * jQuery('#postbox').on('click', _.debounce(sendMail, 300, {
     *   'leading': true,
     *   'trailing': false
     * }));
     *
     * // ensure `batchLog` is invoked once after 1 second of debounced calls
     * var source = new EventSource('/stream');
     * jQuery(source).on('message', _.debounce(batchLog, 250, {
     *   'maxWait': 1000
     * }));
     *
     * // cancel a debounced call
     * var todoChanges = _.debounce(batchLog, 1000);
     * Object.observe(models.todo, todoChanges);
     *
     * Object.observe(models, function(changes) {
     *   if (_.find(changes, { 'user': 'todo', 'type': 'delete'})) {
     *     todoChanges.cancel();
     *   }
     * }, ['delete']);
     *
     * // ...at some point `models.todo` is changed
     * models.todo.completed = true;
     *
     * // ...before 1 second has passed `models.todo` is deleted
     * // which cancels the debounced `todoChanges` call
     * delete models.todo;
     */
    function debounce(func, wait, options) {
      var args,
          maxTimeoutId,
          result,
          stamp,
          thisArg,
          timeoutId,
          trailingCall,
          lastCalled = 0,
          maxWait = false,
          trailing = true;

      if (typeof func != 'function') {
        throw new TypeError(FUNC_ERROR_TEXT);
      }
      wait = wait < 0 ? 0 : (+wait || 0);
      if (options === true) {
        var leading = true;
        trailing = false;
      } else if (isObject(options)) {
        leading = !!options.leading;
        maxWait = 'maxWait' in options && nativeMax(+options.maxWait || 0, wait);
        trailing = 'trailing' in options ? !!options.trailing : trailing;
      }

      function cancel() {
        if (timeoutId) {
          clearTimeout(timeoutId);
        }
        if (maxTimeoutId) {
          clearTimeout(maxTimeoutId);
        }
        lastCalled = 0;
        maxTimeoutId = timeoutId = trailingCall = undefined;
      }

      function complete(isCalled, id) {
        if (id) {
          clearTimeout(id);
        }
        maxTimeoutId = timeoutId = trailingCall = undefined;
        if (isCalled) {
          lastCalled = now();
          result = func.apply(thisArg, args);
          if (!timeoutId && !maxTimeoutId) {
            args = thisArg = undefined;
          }
        }
      }

      function delayed() {
        var remaining = wait - (now() - stamp);
        if (remaining <= 0 || remaining > wait) {
          complete(trailingCall, maxTimeoutId);
        } else {
          timeoutId = setTimeout(delayed, remaining);
        }
      }

      function maxDelayed() {
        complete(trailing, timeoutId);
      }

      function debounced() {
        args = arguments;
        stamp = now();
        thisArg = this;
        trailingCall = trailing && (timeoutId || !leading);

        if (maxWait === false) {
          var leadingCall = leading && !timeoutId;
        } else {
          if (!maxTimeoutId && !leading) {
            lastCalled = stamp;
          }
          var remaining = maxWait - (stamp - lastCalled),
              isCalled = remaining <= 0 || remaining > maxWait;

          if (isCalled) {
            if (maxTimeoutId) {
              maxTimeoutId = clearTimeout(maxTimeoutId);
            }
            lastCalled = stamp;
            result = func.apply(thisArg, args);
          }
          else if (!maxTimeoutId) {
            maxTimeoutId = setTimeout(maxDelayed, remaining);
          }
        }
        if (isCalled && timeoutId) {
          timeoutId = clearTimeout(timeoutId);
        }
        else if (!timeoutId && wait !== maxWait) {
          timeoutId = setTimeout(delayed, wait);
        }
        if (leadingCall) {
          isCalled = true;
          result = func.apply(thisArg, args);
        }
        if (isCalled && !timeoutId && !maxTimeoutId) {
          args = thisArg = undefined;
        }
        return result;
      }
      debounced.cancel = cancel;
      return debounced;
    }

    /**
     * Checks if `value` is the [language type](https://es5.github.io/#x8) of `Object`.
     * (e.g. arrays, functions, objects, regexes, `new Number(0)`, and `new String('')`)
     *
     * @static
     * @memberOf _
     * @category Lang
     * @param {*} value The value to check.
     * @returns {boolean} Returns `true` if `value` is an object, else `false`.
     * @example
     *
     * _.isObject({});
     * // => true
     *
     * _.isObject([1, 2, 3]);
     * // => true
     *
     * _.isObject(1);
     * // => false
     */
    function isObject(value) {
      // Avoid a V8 JIT bug in Chrome 19-20.
      // See https://code.google.com/p/v8/issues/detail?id=2291 for more details.
      var type = typeof value;
      return !!value && (type == 'object' || type == 'function');
    }

    return debounce;

  })();

  // ported lodash throttle function
  var throttle = function( func, wait, options ){
    var leading = true,
        trailing = true;

    if( options === false ){
      leading = false;
    } else if( typeof options === typeof {} ){
      leading = 'leading' in options ? options.leading : leading;
      trailing = 'trailing' in options ? options.trailing : trailing;
    }
    options = options || {};
    options.leading = leading;
    options.maxWait = wait;
    options.trailing = trailing;

    return debounce( func, wait, options );
  };

  var Navigator = function ( element, options ) {
    this._init(element, options)
  };

  var extend = function() {
    for(var i = 1; i < arguments.length; i++) {
      for(var key in arguments[i]) {
        if(arguments[i].hasOwnProperty(key)) {
          arguments[0][key] = arguments[i][key];
        }
      }
    }
    return arguments[0];
  };

  var wid = function(elem) {
    return elem.getBoundingClientRect().width;
  };

  var hei = function(elem) {
    return elem.getBoundingClientRect().height;
  };

  Navigator.prototype = {

    constructor: Navigator

  /****************************
    Main functions
  ****************************/

  , bb: function(){
    var bb = this.cy.elements().boundingBox()

    if( bb.w === 0 || bb.h === 0 ){
      return {
        x1: 0,
        x2: Infinity,
        y1: 0,
        y2: Infinity,
        w: Infinity,
        h: Infinity
      } // => hide interactive overlay
    }

    return bb
  }

  , _addCyListener: function(events, handler){
    this._cyListeners.push({
      events: events,
      handler: handler
    })

    this.cy.on(events, handler)
  }

  , _removeCyListeners: function(){
    var cy = this.cy

    this._cyListeners.forEach(function(l){
      cy.off(l.events, l.handler)
    })

    cy.offRender(this._onRenderHandler)
  }

  , _init: function ( cy, options ) {
      this._cyListeners = []

      this.$element = cy.container()
      this.options = extend({}, defaults, options)

      this.cy = cy

      // Cache bounding box
      this.boundingBox = this.bb()

      // Cache sizes
      this.width = wid(this.$element);
      this.height = hei(this.$element)

      // Init components
      this._initPanel()
      this._initThumbnail()
      this._initView()
      this._initOverlay()
    }

  , destroy: function () {
      this._removeEventsHandling();

      // If container is not created by navigator and its removal is prohibited
      if (this.options.container && !this.options.removeCustomContainer) {
        this.$panel.innerHTML = '';
      } else {
        this.$panel.parentElement.removeChild(this.$panel);
      }
    }

  /****************************
    Navigator elements functions
  ****************************/

    /*
     * Used inner attributes
     *
     * w {number} width
     * h {number} height
     */
  , _initPanel: function () {
      var options = this.options
      if(options.container && typeof options.container === 'string' && options.container.length > 0) {
        // to not break users which gives a jquery string selector
        if (options.container.indexOf('#') !== -1) {
          this.$panel = document.getElementById(options.container.replace('#', ''));
        } else {
          this.$panel = document.getElementsByClassName(options.container.replace('.', ''))[0];
        } 
      } else {
        this.$panel = document.createElement('div');
        this.$panel.className = 'cytoscape-navigator';
        document.body.appendChild(this.$panel);
      }
      this._setupPanel()
      this._addCyListener('resize', this.resize.bind(this))
    }

  , _setupPanel: function () {
      // Cache sizes
      this.panelWidth = wid(this.$panel);
      this.panelHeight = hei(this.$panel);
    }

    /*
     * Used inner attributes
     *
     * zoom {number}
     * pan {object} - {x: 0, y: 0}
     */
  , _initThumbnail: function () {
      // Create thumbnail
      this.$thumbnail = document.createElement('img');

      this.$thumbnail.setAttribute("alt", "Graph navigator");

      // Add thumbnail canvas to the DOM
      this.$panel.appendChild(this.$thumbnail);

      // Setup thumbnail
      this._setupThumbnailSizes()
      this._setupThumbnail()
    }

  , _setupThumbnail: function () {
      this._updateThumbnailImage()
    }

  , _setupThumbnailSizes: function () {
      // Update bounding box cache
      this.boundingBox = this.bb()

      this.thumbnailZoom = Math.min(this.panelHeight / this.boundingBox.h, this.panelWidth / this.boundingBox.w)

      // Used on thumbnail generation
      this.thumbnailPan = {
        x: (this.panelWidth - this.thumbnailZoom * (this.boundingBox.x1 + this.boundingBox.x2))/2
      , y: (this.panelHeight - this.thumbnailZoom * (this.boundingBox.y1 + this.boundingBox.y2))/2
      }
    }

    // If bounding box has changed then update sizes
    // Otherwise just update the thumbnail
  , _checkThumbnailSizesAndUpdate: function () {
      // Cache previous values
      var _zoom = this.thumbnailZoom
        , _pan_x = this.thumbnailPan.x
        , _pan_y = this.thumbnailPan.y

      this._setupThumbnailSizes()

      if (_zoom != this.thumbnailZoom || _pan_x != this.thumbnailPan.x || _pan_y != this.thumbnailPan.y) {
        this._setupThumbnail()
        this._setupView()
      } else {
        this._updateThumbnailImage()
      }
    }

    /*
     * Used inner attributes
     *
     * w {number} width
     * h {number} height
     * x {number}
     * y {number}
     * borderWidth {number}
     * locked {boolean}
     */
  , _initView: function () {
      this.$view = document.createElement('div');
      this.$view.className = 'cytoscape-navigatorView';
      this.$panel.appendChild(this.$view)
      // Compute borders
      this.viewBorderTop = parseInt(this.$view.style['border-top-width'], 10) || 0;
      this.viewBorderRight = parseInt(this.$view.style['border-right-width'], 10) || 0;
      this.viewBorderBottom = parseInt(this.$view.style['border-bottom-width'], 10) || 0;
      this.viewBorderLeft = parseInt(this.$view.style['border-left-width'], 10) || 0;
      
      // Abstract borders
      this.viewBorderHorizontal = this.viewBorderLeft + this.viewBorderRight
      this.viewBorderVertical = this.viewBorderTop + this.viewBorderBottom

      this._setupView()

      // Hook graph zoom and pan
      this._addCyListener('zoom pan', this._setupView.bind(this))
    }

  , _setupView: function () {
      if (this.viewLocked)
        return

      var cyZoom = this.cy.zoom()
        , cyPan = this.cy.pan()

      // Horizontal computation
      this.viewW = this.width / cyZoom * this.thumbnailZoom
      this.viewX = -cyPan.x * this.viewW / this.width + this.thumbnailPan.x - this.viewBorderLeft

      // Vertical computation
      this.viewH = this.height / cyZoom * this.thumbnailZoom
      this.viewY = -cyPan.y * this.viewH / this.height + this.thumbnailPan.y - this.viewBorderTop

      // CSS view
      this.$view.style['width'] = this.viewW + 'px';
      this.$view.style['height'] = this.viewH + 'px';
      this.$view.style['position'] = 'absolute';
      this.$view.style['left'] = this.viewX + 'px';
      this.$view.style['top'] = this.viewY + 'px';
    }

    /*
     * Used inner attributes
     *
     * timeout {number} used to keep stable frame rate
     * lastMoveStartTime {number}
     * inMovement {boolean}
     * hookPoint {object} {x: 0, y: 0}
     */
  , _initOverlay: function () {
      // Used to capture mouse events
      this.$overlay = document.createElement('div');
      this.$overlay.className = 'cytoscape-navigatorOverlay';

      // Add overlay to the DOM
      this.$panel.appendChild(this.$overlay)

      // Init some attributes
      this.overlayHookPointX = 0;
      this.overlayHookPointY = 0;

      // Listen for events
      this._initEventsHandling()
    }

  /****************************
    Event handling functions
  ****************************/

  , resize: function () {
      // Cache sizes
      this.width = wid(this.$element);
      this.height = hei(this.$element);
      this._thumbnailSetup = false
      this._setupPanel()
      this._checkThumbnailSizesAndUpdate()
      this._setupView()
    }

  , _initEventsHandling: function () {
      var that = this
        , eventsLocal = [
        // Mouse events
          'mousedown'
        , 'mousewheel'
        , 'DOMMouseScroll' // Mozilla specific event
        // Touch events
        , 'touchstart'
        ]
        , eventsGlobal = [
          'mouseup'
        , 'mouseout'
        , 'mousemove'
        // Touch events
        , 'touchmove'
        , 'touchend'
        ]

      // handle events and stop their propagation
      var overlayListener = function (ev) {
        // Touch events
        if (ev.type == 'touchstart') {
          // Will count as middle of View
          Object.defineProperty(ev, 'offsetX', {
            value: that.viewX + that.viewW / 2,
            writable: true
          });
          Object.defineProperty(ev, 'offsetY', {
            value: that.viewY + that.viewH / 2,
            writable: true
          });
        }

        // Normalize offset for browsers which do not provide that value
        if (ev.offsetX === undefined || ev.offsetY === undefined) {
          var rect = ev.target.getBoundingClientRect();
          var targetOffset = { 
            top: rect.top + window.scrollY, 
            left: rect.left + window.scrollX, 
          };
          Object.defineProperty(ev, 'offsetX', {
            value: ev.pageX - targetOffset.left,
            writable: true
          });
          Object.defineProperty(ev, 'offsetY', {
            value: ev.pageY - targetOffset.top,
            writable: true
          });
        }

        if (ev.type == 'mousedown' || ev.type == 'touchstart') {
          that._eventMoveStart(ev)
        } else if (ev.type == 'mousewheel' || ev.type == 'DOMMouseScroll') {
          that._eventZoom(ev)
        }

        // Prevent default and propagation
        // Don't use peventPropagation as it breaks mouse events
        return false;
      };

      // Hook global events
      var globalListener = function (ev) {

        // Do not make any computations if it is has no effect on Navigator
        if (!that.overlayInMovement)
          return;

        // Touch events
        if (ev.type == 'touchend') {
          // Will count as middle of View
          Object.defineProperty(ev, 'offsetX', {
            value: that.viewX + that.viewW / 2,
            writable: true
          });
          Object.defineProperty(ev, 'offsetY', {
            value: that.viewY + that.viewH / 2,
            writable: true
          });
        } else if (ev.type == 'touchmove') {
          // Hack - we take in account only first touch
          Object.defineProperty(ev, 'pageX', {
            value: ev.originalEvent.touches[0].pageX,
            writable: true
          });
          Object.defineProperty(ev, 'pageY', {
            value: ev.originalEvent.touches[0].pageY,
            writable: true
          });
        }

        // Normalize offset for browsers which do not provide that value
        if (ev.offsetX === undefined || ev.offsetY === undefined) {
          var rect = ev.target.getBoundingClientRect();
          var targetOffset = { 
            top: rect.top + window.scrollY, 
            left: rect.left + window.scrollX, 
          };
          Object.defineProperty(ev, 'offsetX', {
            value: ev.pageX - targetOffset.left,
            writable: true
          });
          Object.defineProperty(ev, 'offsetY', {
            value: ev.pageY - targetOffset.top,
            writable: true
          });
        }

        // Translate global events into local coordinates
        if (ev.target !== that.$overlay) {
          var rect = ev.target.getBoundingClientRect();
          var rect2 = that.$overlay.getBoundingClientRect();
          var targetOffset = { 
            top: rect.top + window.scrollY, 
            left: rect.left + window.scrollX, 
          };
          var overlayOffset = { 
            top: rect2.top + window.scrollY, 
            left: rect2.left + window.scrollX, 
          };
          
          if(targetOffset && overlayOffset) {
            Object.defineProperty(ev, 'offsetX', {
              value: ev.offsetX - overlayOffset.left + targetOffset.left,
              writable: true
            });
            Object.defineProperty(ev, 'offsetY', {
              value: ev.offsetY - overlayOffset.top + targetOffset.top,
              writable: true
            });
          } else {
            return false;
          }
        }

        if (ev.type == 'mousemove' || ev.type == 'touchmove') {
          that._eventMove(ev)
        } else if (ev.type == 'mouseup' || ev.type == 'touchend') {
          that._eventMoveEnd(ev)
        }

        // Prevent default and propagation
        // Don't use peventPropagation as it breaks mouse events
        return false;
      };

      for (var i = 0; i < eventsLocal.length; i++) {
        this.$overlay.addEventListener(eventsLocal[i], overlayListener, false);
      }

      for (var i = 0; i < eventsGlobal.length; i++) {
        window.addEventListener(eventsGlobal[i], globalListener, false);
      }

      this._removeEventsHandling = function(){

        for (var i = 0; i < eventsLocal.length; i++) {
          this.$overlay.removeEventListener(eventsLocal[i], overlayListener);
        }

        for (var i = 0; i < eventsGlobal.length; i++) {
          window.removeEventListener(eventsGlobal[i], globalListener);
        }
      }
    }

  , _eventMoveStart: function (ev) {
      var now = new Date().getTime()

      // Check if it was double click
      if (this.overlayLastMoveStartTime
        && this.overlayLastMoveStartTime + this.options.dblClickDelay > now) {
        // Reset lastMoveStartTime
        this.overlayLastMoveStartTime = 0
        // Enable View in order to move it to the center
        this.overlayInMovement = true

        // Set hook point as View center
        this.overlayHookPointX = this.viewW / 2
        this.overlayHookPointY = this.viewH / 2

        // Move View to start point
        if (this.options.viewLiveFramerate !== false) {
          this._eventMove({
            offsetX: this.panelWidth / 2
          , offsetY: this.panelHeight / 2
          })
        } else {
          this._eventMoveEnd({
            offsetX: this.panelWidth / 2
          , offsetY: this.panelHeight / 2
          })
        }

        // View should be inactive as we don't want to move it right after double click
        this.overlayInMovement = false
      }
      // This is a single click
      // Take care as single click happens before double click 2 times
      else {
        this.overlayLastMoveStartTime = now
        this.overlayInMovement = true
        // Lock view moving caused by cy events
        this.viewLocked = true

        // if event started in View
        if (ev.offsetX >= this.viewX && ev.offsetX <= this.viewX + this.viewW
          && ev.offsetY >= this.viewY && ev.offsetY <= this.viewY + this.viewH
        ) {
          this.overlayHookPointX = ev.offsetX - this.viewX
          this.overlayHookPointY = ev.offsetY - this.viewY
        }
        // if event started in Thumbnail (outside of View)
        else {
          // Set hook point as View center
          this.overlayHookPointX = this.viewW / 2
          this.overlayHookPointY = this.viewH / 2

          // Move View to start point
          this._eventMove(ev)
        }
      }
    }

  , _eventMove: function (ev) {
      var that = this

      this._checkMousePosition(ev)

      // break if it is useless event
      if (!this.overlayInMovement) {
        return;
      }

      // Update cache
      this.viewX = ev.offsetX - this.overlayHookPointX
      this.viewY = ev.offsetY - this.overlayHookPointY

      // Update view position
      this.$view.style['left'] = this.viewX + 'px';
      this.$view.style['top'] = this.viewY + 'px';

      // Move Cy
      if (this.options.viewLiveFramerate !== false) {
        // trigger instantly
        if (this.options.viewLiveFramerate == 0) {
          this._moveCy()
        }
        // trigger less often than frame rate
        else if (!this.overlayTimeout) {
          // Set a timeout for graph movement
          this.overlayTimeout = setTimeout(function () {
            that._moveCy()
            that.overlayTimeout = false
          }, 1000 / this.options.viewLiveFramerate)
        }
      }
    }

  , _checkMousePosition: function (ev) {
      // If mouse in over View
      if(ev.offsetX > this.viewX && ev.offsetX < this.viewX + this.viewBorderHorizontal + this.viewW
        && ev.offsetY > this.viewY && ev.offsetY < this.viewY + this.viewBorderVertical + this.viewH) {
        this.$panel.classList.add('mouseover-view')
      } else {
        this.$panel.classList.remove('mouseover-view')
      }
    }

  , _eventMoveEnd: function (ev) {
      // Unlock view changing caused by graph events
      this.viewLocked = false

      // Remove class when mouse is not over Navigator
      this.$panel.classList.remove('mouseover-view')

      if (!this.overlayInMovement) {
        return;
      }

      // Trigger one last move
      this._eventMove(ev)

      // If mode is not live then move graph on drag end
      if (this.options.viewLiveFramerate === false) {
        this._moveCy()
      }

      // Stop movement permission
      this.overlayInMovement = false
    }

  , _eventZoom: function (ev) {
      var ev2 = extend({}, ev.originalEvent);
      var delta = ev.wheelDeltaY / 1000 || ev.wheelDelta / 1000 || ev.detail / -32 || ev2.wheelDeltaY / 1000 || ev2.wheelDelta / 1000 || ev2.detail / -32;
      var zoomRate = Math.pow(10, delta)
        , mousePosition = {
            left: ev.offsetX
          , top: ev.offsetY
          }

      if (this.cy.zoomingEnabled()) {
        this._zoomCy(zoomRate, mousePosition)
      }
    }

  , _updateThumbnailImage: function () {
    var that = this;

    if( this._thumbnailUpdating ){
      return;
    }

    this._thumbnailUpdating = true;

    var render = function() {
      that._checkThumbnailSizesAndUpdate();
      that._setupView();

      var $img = that.$thumbnail;
      var img = $img;

      var w = that.panelWidth;
      var h = that.panelHeight;
      var bb = that.boundingBox;
      var zoom = Math.min( w/bb.w, h/bb.h );

      var png = that.cy.png({
        full: true,
        scale: zoom,
        maxHeight: h,
        maxWidth: w
      });
      if( png.indexOf('image/png') < 0 ){
        img.removeAttribute( 'src' );
      } else {
        img.setAttribute( 'src', png );
      }

      var translate = {
        x: (w - zoom*( bb.w ))/2,
        y: (h - zoom*( bb.h ))/2
      };

      $img.style['position'] = 'absolute';
      $img.style['left'] = translate.x + 'px';
      $img.style['top'] = translate.y + 'px';

    }

    this._onRenderHandler = throttle(render, that.options.rerenderDelay)

    this.cy.onRender( this._onRenderHandler )
  }

  /****************************
    Navigator view moving
  ****************************/

  , _moveCy: function () {
      this.cy.pan({
        x: -(this.viewX + this.viewBorderLeft - this.thumbnailPan.x) * this.width / this.viewW
      , y: -(this.viewY + this.viewBorderLeft - this.thumbnailPan.y) * this.height / this.viewH
      })
    }

  /**
   * Zooms graph.
   *
   * @this {cytoscapeNavigator}
   * @param {number} zoomRate The zoom rate value. 1 is 100%.
   */
  , _zoomCy: function (zoomRate, zoomCenterRaw) {
      var zoomCenter
        , isZoomCenterInView = false

      zoomCenter = {
        x: this.width / 2
      , y: this.height / 2
      };

      this.cy.zoom({
        level: this.cy.zoom() * zoomRate
      , renderedPosition: zoomCenter
      })
    }
  }

  // registers the extension on a cytoscape lib ref
  var register = function( cytoscape ){

    if (!cytoscape){ return; } // can't register if cytoscape unspecified

    cytoscape( 'core', 'navigator', function( options ){
      var cy = this;

      return new Navigator( cy, options );
    } );

  };

  if (typeof module !== 'undefined' && module.exports) { // expose as a commonjs module
    module.exports = function( cytoscape ){
      register( cytoscape );
    };
  } else if (typeof define !== 'undefined' && define.amd) { // expose as an amd/requirejs module
    define('cytoscape-navigator', function(){
      return register;
    });
  }

  if (typeof cytoscape !== 'undefined') { // expose to global cytoscape (i.e. window.cytoscape)
    register(cytoscape);
  }

})();
