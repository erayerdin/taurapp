var buddyList = document.getElementsByClassName('_1ht1');

function clickHandler(buddy) {
  var reactId = buddy.getAttribute('data-reactid');
  window.open('https://www.messenger.com', reactId);
}

for(var i = 0; i < buddyList.length; i++) {
  var buddy = buddyList[i];
  buddy.addEventListener('dblclick', clickHandler.bind(this, buddy));
}
