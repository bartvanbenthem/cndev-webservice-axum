Handlebars.registerHelper('mod', (a, b) => a % b);
Handlebars.registerHelper('eq', (a, b) => a === b);

function fetchJSON(url) {
  return fetch(url).then(res => res.json());
}

function truncateContent(content, wordLimit) {
  return content.split(' ').slice(0, wordLimit).join(' ') + '...';
}

function renderTemplate(templateId, targetId, context) {
  const source = document.getElementById(templateId).innerHTML;
  const template = Handlebars.compile(source);
  document.getElementById(targetId).innerHTML = template(context);
}

async function listPosts() {
  const data = await fetchJSON('/api/v1/content/posts');
  const posts = data.reverse().slice(0,2).map(p => ({...p, summary: truncateContent(p.content,35)}));
  renderTemplate('post-template', 'postList', { posts });
}

async function showAbout() {
  const data = await fetchJSON('/api/v1/content/about');
  renderTemplate('about-template', 'showAbout', data);
}

async function listServices() {
  const data = await fetchJSON('/api/v1/content/services');
  const services = data.slice(0,6).map((s,i) => ({...s, summary: s.summary, odd: i%2!==0}));
  renderTemplate('services-template', 'serviceList', { services });
}

document.addEventListener('DOMContentLoaded', () => {
  listPosts();
  showAbout();
  listServices();
});

document.getElementById('contactForm').addEventListener('submit', async function(e){
  e.preventDefault();
  clearErrors();
  let isValid = true;
  const name = document.getElementById('name').value.trim();
  const email = document.getElementById('from').value.trim();
  const phone = document.getElementById('phone').value.trim();
  const subject = document.getElementById('subject').value.trim();
  const message = document.getElementById('message').value.trim();

  if(!name){displayError('nameError','Name is required'); isValid=false;}
  if(!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)){displayError('emailError','Give valid email adress'); isValid=false;}
  if(!/^\d{10}$/.test(phone)){displayError('phoneError','Give valid phone number (10 digits)'); isValid=false;}
  if(!subject){displayError('subjectError','Subject is required'); isValid=false;}
  if(!message){displayError('messageError','Message is required'); isValid=false;}

  if(!isValid) return;

  const formData = new URLSearchParams(new FormData(this));
  try{
    const response = await fetch('api/v1/mail/send',{
      method:'POST',
      headers:{'Content-Type':'application/x-www-form-urlencoded'},
      body:formData
    });
    if(response.ok){
      alert('Thank you for sending us the email, we will respond soon.');
      this.reset();
    }else{
      alert('Fault while sending: '+await response.text());
    }
  }catch(err){
    console.error('Error:',err);
    alert('A fault occured sending the message, try again later.');
  }
});

function displayError(id,msg){document.getElementById(id).textContent=msg;}
function clearErrors(){['nameError','emailError','phoneError','subjectError','messageError'].forEach(id=>document.getElementById(id).textContent='');}