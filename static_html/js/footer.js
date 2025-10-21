document.addEventListener("DOMContentLoaded", function () {
    const footerContent = `
        <footer class="text-white text-center text-lg-start tile-darkgreen mt-auto">
            <!-- Grid container -->
            <div class="container p-4">
                <div class="row">
                    <!-- Location Column -->
                    <div class="col-lg-4 col-md-6 mb-4 mb-md-0 text-center">
                        <h3 class="text-dark">Locaties</h3>
                        <p>streetone 3, 6881 KX City 1
                        <br>streettwo 2, 6812 DC City 2</p>
                    </div>

                    <!-- Contact Column -->
                    <div class="col-lg-4 col-md-6 mb-4 mb-md-0 text-center">
                        <h3 class="text-dark">Make Appointment</h3>
                        <a class="link-light" href="#contact">Make contact first</a></br>
                        <a class="link-light" href="https://wa.me/+31600000000?text=Hallo CNDEV,%20ik%20wil%20graag%20een%20afspraak%20plannen%20in%20loc1."> Appointment in Location 1</a></br>
                        <a class="link-light" href="https://wa.me/+31600000000?text=Hallo CNDEV,%20ik%20wil%20graag%20een%20afspraak%20plannen%20in%20loc2."> Appointment in Location 2</a></br>
                        </br>
                    </div>

                    <!-- Partners Column -->
                    <div class="col-lg-4 col-md-6 mb-4 mb-md-0 text-center d-none d-md-block">
                        <h3 class="text-dark">Samenwerking</h3>
                        <a class="link-light" href="https://github.com/bartvanbenthem/cndev-webservice-axum">Partner site 1</a></br>
                        <a class="link-light" href="https://github.com/bartvanbenthem/cndev-webservice-axum">Partner site 2</a></br>
                        <a class="link-light" href="https://github.com/bartvanbenthem/cndev-webservice-axum">Partner site 3</a></br>
                    </div>
                </div>
            </div>

            <!-- Copyright -->
            <div class="text-center p-3 text-white tile-darkblue">
                © 2024 PoweredBy:
                <a class="text-white" href="https://github.com/bartvanbenthem/cndev-webservice-axum">cndev.nl</a>
            </div>
        </footer>
    `;

    const footerContainer = document.getElementById("footerContainer");
    if (footerContainer) {
        footerContainer.innerHTML = footerContent;
    }
});
