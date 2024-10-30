$(document).ready(function () {
    const footerContent = `
        <footer class="text-white text-center text-lg-start tile-darkgreen mt-auto">
            <!-- Grid container -->
            <div class="container p-4">
                <div class="row">
                    <!-- Location Column -->
                    <div class="col-lg-4 col-md-6 mb-4 mb-md-0 text-center">
                        <h3 class="text-dark">Locatie</h3>
                        <p>Zypendaalse weg 999<br>Arnhem, 6828 LK<br>Sonsbeek Park</p>
                    </div>

                    <!-- Contact Column -->
                    <div class="col-lg-4 col-md-6 mb-4 mb-md-0 text-center">
                        <h3 class="text-dark">make contact</h3>
                        <a class="link-light" href="#contact">Neem eerst contact op</a><br>
                        <a class="link-light" href="https://github.com/bartvanbenthem/">Plan direct een afspraak</a><br>
                        <br>
                    </div>

                    <!-- Partners Column -->
                    <div class="col-lg-4 col-md-6 mb-4 mb-md-0 text-center d-none d-md-block">
                        <h3 class="text-dark">Partners</h3>
                        <a class="link-light" href="https://github.com/bartvanbenthem/">[ test ]</a><br>
                        <a class="link-light" href="https://github.com/bartvanbenthem/">[ another test ]</a><br>
                        <a class="link-light" href="https://github.com/bartvanbenthem/">[ another test ]</a><br>
                    </div>
                </div>
            </div>

            <!-- Copyright -->
            <div class="text-center p-3 text-white tile-darkblue">
                <p class="fw-light">KvK nr. 00000001</p>
                © 2024 PoweredBy:
                <a class="text-white" href="https://github.com/bartvanbenthem/cndev-webservice-axum">cndev.nl</a>
            </div>
        </footer>
    `;

    // Insert the footer HTML into the specified container
    $("#footerContainer").html(footerContent);
});